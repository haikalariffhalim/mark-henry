use car;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{Document, Element, HtmlElement, Node, NodeList, window};

#[wasm_bindgen(start)]
pub fn start() {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let closure = Closure::wrap(Box::new(move || {
        if let Some(main) = document.query_selector("main").unwrap() {
            convert_to_nested_sections(&main);
        }
        add_parent_heading_attribute(&document);
        start_nav_observation(&document);
        setup_menu_buttons(&document);
    }) as Box<dyn Fn()>);

    window
        .add_event_listener_with_callback("DOMContentLoaded", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}

fn convert_to_nested_sections(root_element: &Element) {
    let children = root_element.children();
    let length = children.length();

    // Collect children into a Vec<Element>
    let mut elements = Vec::new();
    for i in 0..length {
        if let Some(child) = children.item(i) {
            elements.push(child);
        }
    }

    // Remove all children from root_element
    for element in &elements {
        root_element.remove_child(element).unwrap();
    }

    let mut current_section = root_element.clone();
    let mut current_level = 0;

    for element in elements {
        let tag_name = element.tag_name();
        let heading_match =
            if tag_name.len() == 2 && (tag_name.starts_with('H') || tag_name.starts_with('h')) {
                tag_name.chars().nth(1).and_then(|c| c.to_digit(10))
            } else {
                None
            };

        if let Some(new_level) = heading_match {
            let new_level = new_level as i32;

            // while currentLevel + 1 < newLevel
            while current_level + 1 < new_level {
                let section = root_element
                    .owner_document()
                    .unwrap()
                    .create_element("section")
                    .unwrap();
                current_section.append_child(&section).unwrap();
                current_section = section;
                current_level += 1;
            }

            // while currentLevel + 1 > newLevel
            while current_level + 1 > new_level {
                if let Some(parent) = current_section.parent_node() {
                    if let Some(parent_element) = parent.dyn_ref::<Element>() {
                        current_section = parent_element.clone();
                        current_level -= 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            let id = element.get_attribute("id");

            let new_section = root_element
                .owner_document()
                .unwrap()
                .create_element("section")
                .unwrap();
            if let Some(id) = id {
                new_section.set_attribute("id", &id).unwrap();
                element.remove_attribute("id").unwrap();

                let permalink = root_element
                    .owner_document()
                    .unwrap()
                    .create_element("a")
                    .unwrap();
                permalink
                    .set_attribute("href", &format!("#{}", id))
                    .unwrap();
                permalink.class_list().add_1("permalink").unwrap();
                element.append_child(&permalink).unwrap();
            }

            current_section.append_child(&new_section).unwrap();

            current_section = new_section;
            current_level = new_level;
        }

        current_section.append_child(&element).unwrap();
    }
}

fn add_parent_heading_attribute(document: &Document) {
    let selector = "h1,h2,h3,h4,h5,h6";
    let headings = document.query_selector_all(selector).unwrap();

    for i in 0..headings.length() {
        if let Some(heading) = headings.item(i) {
            if let Some(parent_element) = heading.parent_element() {
                if let Some(grandparent) = parent_element.parent_element() {
                    if let Ok(parent_heading) = grandparent.query_selector(selector) {
                        if let Some(parent_heading) = parent_heading {
                            let text_content = parent_heading.text_content().unwrap_or_default();
                            heading
                                .set_attribute("data-parent-heading", &text_content)
                                .unwrap();
                        }
                    }
                }
            }
        }
    }
}

fn highlight_first_active(document: &Document) {
    let nav_li = document.query_selector_all("nav li").unwrap();
    for i in 0..nav_li.length() {
        if let Some(link) = nav_li.item(i) {
            link.class_list().remove_1("active").unwrap();
        }
    }

    if let Some(first_visible_link) = document.query_selector("nav li.visible").unwrap() {
        if let Ok(first_visible_child) = first_visible_link.query_selector("li.visible") {
            if let Some(child) = first_visible_child {
                child.class_list().add_1("active").unwrap();
            } else {
                first_visible_link.class_list().add_1("active").unwrap();
            }
        }
    }
}

fn start_nav_observation(document: &Document) {
    let closure = Closure::wrap(Box::new(move |entries: js_sys::Array| {
        let document = window().unwrap().document().unwrap();

        for entry in entries.iter() {
            let entry = entry.unchecked_ref::<web_sys::IntersectionObserverEntry>();
            let target = entry.target();

            if let Some(id) = target.get_attribute("id") {
                let selector = format!("nav li a[href=\"#{}\"]", id);
                if let Ok(Some(link)) = document.query_selector(&selector) {
                    if let Some(parent_element) = link.parent_element() {
                        if entry.intersection_ratio() > 0.0 {
                            parent_element.class_list().add_1("visible").unwrap();
                        } else {
                            parent_element.class_list().remove_1("visible").unwrap();
                        }
                    }
                }
            }
        }
        highlight_first_active(&document);
    }) as Box<dyn FnMut(js_sys::Array)>);

    let observer = web_sys::IntersectionObserver::new(closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();

    let sections = document.query_selector_all("section[id]").unwrap();
    for i in 0..sections.length() {
        if let Some(section) = sections.item(i) {
            observer.observe(&section);
        }
    }
}

fn setup_menu_buttons(document: &Document) {
    if let Some(button) = document.query_selector("#menu-button") {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            if let Some(body) = window().unwrap().document().unwrap().body() {
                body.class_list().add_1("menu-open").unwrap();
            }
        }) as Box<dyn FnMut(_)>);

        button
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    if let Some(menu) = document.query_selector("#TableOfContents") {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            if let Some(body) = window().unwrap().document().unwrap().body() {
                body.class_list().remove_1("menu-open").unwrap();
            }
        }) as Box<dyn FnMut(_)>);

        menu.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}
