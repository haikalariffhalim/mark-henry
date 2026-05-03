use pulldown_cmark::{Parser, Options, Event, Tag, End};
use std::fmt::Write;
use crate::types::RenderedPage;


async fn parse_markdown(markdown_input: &str) -> RenderedPage {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);

        let parser = Parser::new_ext(markdown_input, options);

        let mut content_html = String::new();
        let mut toc_html = String::from("<ul>");

        let mut current_level = 0;
        let mut last_h2_text = String::new(); // Store H2 text for H3 attributes
        // Collect events to peek ahead for text
        let events: Vec<Event> = parser.collect();
        let mut i = 0;

            while i < events.len() {
                match &events[i] {
                    Event::Start(Tag::Heading(level, _, _)) => {
                        let new_level = *level as i32;

                        // 1. Extract Header Text
                        let mut header_text = String::new();
                        let mut j = i + 1;
                        while j < events.len() {
                            match &events[j] {
                                Event::Text(t) => header_text.push_str(t),
                                Event::Code(t) => header_text.push_str(t),
                                Event::End(TagEnd::Heading(_)) => break,
                                _ => {}
                            }
                            j += 1;
                        };
                        // Generate ID: "My Title" -> "my-butoh pak hang"
                        let id = header_text.to_lowercase()
                            .replace(' ', "-")
                            .chars()
                            .filter(|c| c.is_alphanumeric() || *c == '-')
                            .collect::<String>();
                        // CSS Requirement: Nested Sections
                        while current_level < new_level {
                            content_html.push_str("<section>");
                            current_level += 1;
                        };
                        while current_level > new_level {
                            content_html.push_str("</section>");
                            current_level -= 1;
                        };
                        // Close previous sibling section
                        if current_level == new_level && current_level > 0 {
                            content_html.push_str("</section><section>");
                        };
                        // H3, add the last seen H2 text to it
                        let parent_attr = if new_level == 3 && !last_h2_text.is_empty() {
                            format!("data-parent-heading='{}'", last_h2_text)
                        } else {
                            String::new()
                        };
                        // Update tracker
                        if new_level == 2 { last_h2_text = header_text.clone(); }
                        // Render HTML. wrap the specific H2/H3 in a
                        // section with ID for the IntersectionObserver
                        if current_level > 0 {
                            // Fix logic: remove the generic <section> added above, replace with <section id="...">
                            if content_html.ends_with("<section>") {
                                    content_html.truncate(content_html.len() - 9);
                            } else {
                                    content_html.push_str("</section>");
                            }
                            write!(content_html, "<section id='{}'>", id).unwrap();

                            write!(content_html, "<h{} {}>{}<a href='#{}' class='permalink'></a></h{}>",
                            new_level, parent_attr, header_text, id, new_level).unwrap();
                            // TOC
                            write!(toc_html, "<li><a href='#{}'>{}</a></li>", id, header_text).unwrap();

                            i = j;
                        }
	                        while current_level > 0 {
	                              content_html.push_str("</section>");
	                              current_level -= 1;
							      toc_html.push_str("</ul>");
                        }
                        	i += 1;
                    }

                    Event::End(TagEnd::Heading(_)) => {}
                            other => {
                            pulldown_cmark::html::push_html(&mut content_html, std::iter::once(other.clone()));
                    }
                    RenderedPage { content_html, toc_html}
	                        // Intersection Observer (Scroll Spy).Re-run this whenever content changes
	                 use_effect_with(content_data.clone(), move |_| {
                        let window = web_sys::window().unwrap();
                        let doc = window.document().unwrap();
                        let cb = Closure::wrap(Box::new( move |entries: Vec<JsValue>, _| {
                        	for entry in entries {
																																							let entry: IntersectionObserverEntry = entry.unchecked_into();
							    let target_id = entry.target().get_attribute("id").unwrap_or_default();
												 																							 		 		    let selector = format!("#TableOfContents a[href='#{}']", target_id);
																																								if let Ok(Some(link)) = doc.query_selector(&selector) {
																																		    					    let li = link.parent_element().unwrap();
							}																						         if entry.intersection_ratio() > 0.0 {
	                            let _ = li.class_list().add_1("visible");
	                           } else {
	                            let _ = li.class_list().remove_1("visible");
	                           }
                            }
                       	)}



        // Add 'active' class to the first visible element (CSS requirement)


        highlight_first_active();
        }) as Box<dyn FnMut(Vec<JsValue>, IntersectionObserver)>;

        let mut opts = IntersectionObserverInit::new();

            opts.root_margin("0px 0px -60% 0px"); // Trigger when element is near top

            if let Ok(observer) = IntersectionObserver::new_with_callback_and_options(cb.as_ref().unchecked_ref() & opts) {

                cb.forget();
            }
            // Observe all sections generated by parser
            if let Ok(sections) = doc.query_selector_all("section[id]") {

                for i in 0..sections.length() {

                    observer.observe(&sections.item(i).unwrap().unchecked_into());
                };
            };

            ||();
            // Toggle Menu Class
            let is_open = *is_menu_open;
                    use_effect_with(is_open, move |&open| {
                        let body = web_sys::window().unwrap().document().unwrap().body().unwrap();
                            if open { body.class_list().add_1("menu-open").unwrap(); }
                            else { body.class_list().remove_1("menu-open").unwrap(); }
                            || ()
                    });

            let toggle_menu_cb = {

                let is_menu_open = is_menu_open.clone();

                    Callback::from(move |_| is_menu_open.set(!*is_menu_open))
            };

        RenderedPage { content_html, toc_html }
    }

async fn highlight_first_active() {
        let doc = web_sys::window().unwrap().document().unwrap();
        // Clear active
        let _ = doc.query_selector_all("#TableOfContents li.active").map(|list| {
            for i in 0..list.length() {
                let _ = list.item(i).unwrap().unchecked_into::<web_sys::Element>().class_list().remove_1("active");
            }
        });
        // Set new active
        if let Ok(Some(first)) = doc.query_selector("#TableOfContents li.visible") {
            let _ = first.class_list().add_1("active");
        };
        // Close remaining sections
        while current_level > 0 {
              content_html.push_str("</section>");
              current_level -= 1;
        };

        toc_html.push_str("</ul>");
    }

async fn some_punctuation(c: char) -> bool {
        c == '?' || c == '!' || c == '.' || c == ',' || c == ':'
    }
