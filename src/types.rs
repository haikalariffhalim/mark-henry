use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct MenuItem {
    pub title: String,
    pub path: String,
}

#[derive(Clone, PartialEq)]
pub struct RenderedPage {
    pub content_html: String,
    pub toc_html: String,
}
