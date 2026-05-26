use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn highlight_first_active() {
    let window = web_sys::window().expect("no global `window` exists");
    let doc = window.document().expect("should have a document on window");

    let _ = doc
        .query_selector_all("#TableOfContents li.active")
        .map(|list| {
            for i in 0..list.length() {
                let _ = list
                    .item(i)
                    .unwrap()
                    .unchecked_into::<HtmlElement>()
                    .class_list()
                    .remove_1("active");
            }
        });
    if let Ok(Some(first)) = doc.query_selector("#TableOfContents li.visible") {
        let _ = first.class_list().add_1("active");
    }
}
