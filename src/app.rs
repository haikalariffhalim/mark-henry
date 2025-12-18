use gloo::net::http::Request;
use js_sys::Date;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use yew::prelude::*;

use crate::AppConfig; // From lib.rs
use crate::parser; // From parser.rs
use crate::types::MenuItem; // From types.rs
use crate::utils; // From utils.rs

#[function_component(App)]
pub fn app(props: &AppConfig) -> Html {
    // STATE: Holds the HTML for the current articles/topics
    let content_data = use_state(|| crate::types::RenderedPage {
        content_html: "<div style='margin-top:20vh; text-align:center'>Select a chapter...</div>"
            .to_string(),
        toc_html: "".to_string(),
    });
    // Holds the list of topics (Sidebar)
    let menu_items = use_state(|| Vec::<MenuItem>::new());
    // Holds mobile menu state
    let is_menu_open = use_state(|| false);
    // FETCH MENU (Runs once on start)
    {
        let menu_items = menu_items.clone();
        let user = props.user.clone();
        let repo = props.repo.clone();
        let branch = props.branch.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                // Construct URL using the Props (Dynamic Config)
                let url = format!(
                    "https://raw.githubusercontent.com/{}/{}/{}/menu.json?t={}",
                    user,
                    repo,
                    branch,
                    Date::now()
                );

                if let Ok(resp) = Request::get(&url).send().await {
                    if let Ok(data) = resp.json::<Vec<MenuItem>>().await {
                        menu_items.set(data);
                    }
                }
            });
            || ()
        });
    }

    // 3. LOAD CHAPTER FUNCTION (Runs when you click a link)
    let load_chapter = {
        let content_data = content_data.clone();
        let is_menu_open = is_menu_open.clone();
        let user = props.user.clone();
        let repo = props.repo.clone();
        let branch = props.branch.clone();

        Callback::from(move |path: String| {
            let content_data = content_data.clone();
            let is_menu_open = is_menu_open.clone();
            let user = user.clone();
            let repo = repo.clone();
            let branch = branch.clone();
            // Close mobile menu
            is_menu_open.set(false);

            spawn_local(async move {
                // Show loading state
                content_data.set(crate::types::RenderedPage {
                    content_html: "<div class='loading'>Loading...</div>".to_string(),
                    toc_html: "".to_string(),
                });
                let url = format!(
                    "https://raw.githubusercontent.com/{}/{}/{}/{}?t={}",
                    user,
                    repo,
                    branch,
                    path,
                    Date::now()
                );
                // nanti check : Rust should know 'resp' is a Response motherfucker
                // nanti check: "This variable Response or an Error"
                let fetch_result: Result<gloo::net::http::Response, _> =
                    Request::get(&url).send().await;
                match fetch_result {
                    Ok(resp) => {
                        // nanti check : text() should works (DONE)
                        let text = resp.text().await.unwrap_or_default();
                        let parsed = parser::parse_markdown(&text);
                        content_data.set(crate::parsed_markdown::RenderedPage);
                    }
                    Err(_) => {
                        content_data.set(crate::types::RenderedPage);

                        //content_html = "<h1>Error</h1><p>Failed to fetch content.</p>".to_string();
                        //toc_html = "".to_string();
                    }
                }
            });
        })
    };
    // 4. SCROLL SPY (Intersection Observer)
    use_effect_with(content_data.clone(), move |_| {
        let window = web_sys::window().unwrap();
        let doc = window.document().unwrap();

        let cb = Closure::wrap(Box::new(move |entries: Vec<JsValue>, _| {
            for entry in entries {
                let entry: IntersectionObserverEntry = entry.unchecked_into();
                let target_id = entry.target().get_attribute("id").unwrap_or_default();

                let selector = format!("#TableOfContents a[href='#{}']", target_id);
                if let Ok(Some(link)) = doc.query_selector(&selector) {
                    let li = link.parent_element().unwrap();
                    if entry.intersection_ratio() > 0.0 {
                        let _ = li.class_list().add_1("visible");
                    } else {
                        let _ = li.class_list().remove_1("visible");
                    }
                }
            }
            // check: utils target (DONE)
            utils::highlight_first_active();
        })
            as Box<dyn FnMut(Vec<JsValue>, IntersectionObserver)>);
        // check: mut tak boleh mut self (DONE MOTHERFUCKERRR)
        let mut opts = IntersectionObserverInit::new();
        opts.root_margin("0px 0px -70% 0px");
        //
        if let Ok(observer) =
            IntersectionObserver::new_callback_with_opts(cb.as_ref().unchecked_ref(), &opts)
        {
            cb.forget();

            if let Ok(sections) = doc.query_selector_all("section[id]") {
                for i in 0..sections.length() {
                    observer.observe(&sections.item(i).unwrap().unchecked_into());
                }
            }
        }
        || ()
    });
    // SMOL
    {
        let is_open = *is_menu_open;
        use_effect_with(is_open, move |&open| {
            let body = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap();
            if open {
                body.class_list().add_1("menu-open").unwrap();
            } else {
                body.class_list().remove_1("menu-open").unwrap();
            }
            || ()
        });
    }

    let toggle_menu_cb = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| is_menu_open.set(!*is_menu_open))
    };

    // 6. RENDER HTML
    html! {
        <>
            <button id="menu-button" onclick={toggle_menu_cb}>{"Contents"}</button>

            <nav id="TableOfContents">
                <div class="repo-menu">
                    <h3>{"Chapters"}</h3>
                    <ul>
                    { for menu_items.iter().map(|item| {
                        let path = item.path.clone();
                        let load = load_chapter.clone();
                        html! { <li onclick={ move |_| load.emit(path.clone())}><a href="#">{ &item.title }</a></li> }
                    })}
                    </ul>
                </div>

                <hr style="margin: 2em 0; opacity: 0.1"/>

                <div class="page-toc">
                     { Html::from_html_unchecked(AttrValue::from(content_data.toc_html.clone())) }
                </div>
            </nav>

            <main>
                { Html::from_html_unchecked(AttrValue::from(content_data.content_html.clone())) }
            </main>
        </>
    }
}
