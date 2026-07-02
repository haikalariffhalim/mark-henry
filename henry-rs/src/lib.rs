use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app;
mod parser;
mod types;
mod utils;

#[derive(Clone, PartialEq, Properties)]
struct AppConfig {
    pub user: String,
    pub repo: String,
    pub branch: String,
}

#[wasm_bindgen(start)]
pub fn run_app(user: String, repo: String, branch: String) {
    let props = AppConfig { user, repo, branch };
    yew::Renderer::<app::App>::with_props(props).render();
}
