use crate::types::RenderedPage;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};
use std::fmt::Write;

/// Convert a header text into a slug suitable for an id attribute
fn slugify(s: &str) -> String {
    s.to_lowercase()
        .trim()
        .replace(|c: char| c.is_whitespace(), "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}

/// Parse markdown and return rendered HTML content and a table of contents.
/// This is intentionally synchronous so callers can use it without awaiting.
pub fn parse_markdown(markdown_input: &str) -> RenderedPage {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(markdown_input, options);

    let events: Vec<Event> = parser.collect();

    let mut content_html = String::new();
    let mut toc_html = String::from("<ul>");

    let mut pending: Vec<Event> = Vec::new();

    let mut i = 0usize;
    let mut last_h2_text = String::new();
    let mut h2_open = false;

    while i < events.len() {
        match &events[i] {
            Event::Start(Tag::Heading(level, _, _)) => {
                // flush pending non-heading content
                if !pending.is_empty() {
                    pulldown_cmark::html::push_html(&mut content_html, pending.into_iter());
                    pending = Vec::new();
                }

                let mut header_text = String::new();
                let mut j = i + 1;
                while j < events.len() {
                    match &events[j] {
                        Event::Text(t) | Event::Code(t) => header_text.push_str(t),
                        Event::End(Tag::Heading(_, _, _)) => break,
                        _ => {}
                    }
                    j += 1;
                }

                let id = slugify(&header_text);
                let level_num = *level;

                match level_num {
                    HeadingLevel::H1 => {
                        // Close any open H2 section before opening H1
                        if h2_open {
                            content_html.push_str("</section>");
                            h2_open = false;
                        }
                        write!(
                            content_html,
                            "<h1 id='{}'>{}<a href='#{}' class='permalink'></a></h1>",
                            id, header_text, id
                        )
                        .unwrap();
                        write!(toc_html, "<li><a href='#{}'>{}</a></li>", id, header_text).unwrap();
                    }
                    HeadingLevel::H2 => {
                        if h2_open {
                            content_html.push_str("</section>");
                        }
                        write!(content_html, "<section id='{}'>", id).unwrap();
                        write!(
                            content_html,
                            "<h2>{}<a href='#{}' class='permalink'></a></h2>",
                            header_text, id
                        )
                        .unwrap();
                        last_h2_text = header_text.clone();
                        h2_open = true;

                        write!(toc_html, "<li><a href='#{}'>{}</a></li>", id, header_text).unwrap();
                    }
                    HeadingLevel::H3 => {
                        if !last_h2_text.is_empty() {
                            write!(
                                content_html,
                                "<section id='{}' data-parent-heading='{}'>",
                                id,
                                html_escape::encode_double_quoted_attribute(&last_h2_text)
                            )
                            .unwrap();
                        } else {
                            write!(content_html, "<section id='{}'>", id).unwrap();
                        }
                        write!(
                            content_html,
                            "<h3>{}<a href='#{}' class='permalink'></a></h3>",
                            header_text, id
                        )
                        .unwrap();
                        content_html.push_str("</section>");

                        write!(
                            toc_html,
                            "<li class='toc-h3'><a href='#{}'>{}</a></li>",
                            id, header_text
                        )
                        .unwrap();
                    }
                    HeadingLevel::H4 | HeadingLevel::H5 | HeadingLevel::H6 => {
                        let level_str = match level_num {
                            HeadingLevel::H4 => "4",
                            HeadingLevel::H5 => "5",
                            HeadingLevel::H6 => "6",
                            _ => unreachable!(),
                        };
                        write!(
                            content_html,
                            "<h{} id='{}'>{}<a href='#{}' class='permalink'></a></h{}>",
                            level_str, id, header_text, id, level_str
                        )
                        .unwrap();
                        // Don't include H4+ in TOC
                    }
                }
                let mut k = j;
                while k < events.len() {
                    if let Event::End(Tag::Heading(_, _, _)) = &events[k] {
                        break;
                    }
                    k += 1;
                }
                i = k + 1;
            }
            other => {
                pending.push(other.clone());
                i += 1;
            }
        }
    }
    if !pending.is_empty() {
        pulldown_cmark::html::push_html(&mut content_html, pending.into_iter());
    }
    if h2_open {
        content_html.push_str("</section>");
    }
    toc_html.push_str("</ul>");
    RenderedPage {
        content_html,
        toc_html,
    }
}
