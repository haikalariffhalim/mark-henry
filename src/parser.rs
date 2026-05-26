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

    // buffer for non-heading events to be flushed as HTML
    let mut pending: Vec<Event> = Vec::new();

    let mut i = 0usize;
    let mut last_h2_text = String::new();
    let mut h2_open = false; // whether we have an open H2 <section>

    while i < events.len() {
        match &events[i] {
            Event::Start(Tag::Heading(level, _, _)) => {
                // flush pending non-heading content
                if !pending.is_empty() {
                    pulldown_cmark::html::push_html(&mut content_html, pending.into_iter());
                    pending = Vec::new();
                }

                // collect inner text of heading
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
                let level_num = *level; // u32

                match level_num {
                    h2 => {
                        // close previous H2 section if open
                        if h2_open {
                            content_html.push_str("</section>");
                        }
                        // open new section with id
                        write!(content_html, "<section id=\'{}\'>", id).unwrap();
                        write!(
                            content_html,
                            "<h2>{}<a href=\'#{}\' class=\'permalink\'></a></h2>",
                            header_text, id
                        )
                        .unwrap();
                        // record last h2 text for possible H3 parent attribute
                        last_h2_text = header_text.clone();
                        h2_open = true;

                        // add to TOC
                        write!(toc_html, "<li><a href=\'#{}\'>{}</a></li>", id, header_text)
                            .unwrap();
                    }
                    pulldown_cmark::HeadingLevel::H3 => {
                        // render H3 as its own section with reference to parent H2
                        if !last_h2_text.is_empty() {
                            write!(
                                content_html,
                                "<section id=\'{}\' data-parent-heading=\'{}\'>",
                                id,
                                html_escape::encode_double_quoted_attribute(&last_h2_text)
                            )
                            .unwrap();
                        } else {
                            write!(content_html, "<section id=\'{}\'>", id).unwrap();
                        }
                        write!(
                            content_html,
                            "<h3>{}<a href=\'#{}\' class=\'permalink\'></a></h3>",
                            header_text, id
                        )
                        .unwrap();
                        content_html.push_str("</section>");

                        // add to TOC
                        write!(
                            toc_html,
                            "<li class=\'toc-h3\'><a href=\'#{}\'>{}</a></li>",
                            id, header_text
                        )
                        .unwrap();
                    }
                    pulldown_cmark::HeadingLevel::H4 => {
                        // For other heading levels, emit a normal heading with id
                        write!(
                            content_html,
                            "<h4 id=\'{}\'>{}<a href=\'#{}\' class=\'permalink\'></a></h4>",
                            id, header_text, id
                        )
                        .unwrap();
                        // include in TOC only for h1-h3? we'll include h1 as top-level
                        if level_num == HeadingLevel::H1
                            || level_num == HeadingLevel::H2
                            || level_num == HeadingLevel::H3
                        {
                            write!(
                                toc_html,
                                "<li class=\'toc-h1\'><a href=\'#{}\'>{}</a></li>",
                                id, header_text
                            )
                            .unwrap();
                        }
                    }
                }

                // advance i to after the End(Heading)
                // find the matching End
                let mut k = j;
                while k < events.len() {
                    if let Event::End(Tag::Heading(_, _, _)) = &events[k] {
                        break;
                    }
                    k += 1;
                }
                i = k + 1; // continue after the End
            }
            other => {
                // buffer non-heading events
                pending.push(other.clone());
                i += 1;
            }
        }
    }

    // flush remaining pending
    if !pending.is_empty() {
        pulldown_cmark::html::push_html(&mut content_html, pending.into_iter());
    }

    // close any open H2 section
    if h2_open {
        content_html.push_str("</section>");
    }

    toc_html.push_str("</ul>");

    RenderedPage {
        content_html,
        toc_html,
    }
}
