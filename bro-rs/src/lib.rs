use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod app;
pub mod parser;
pub mod types;
pub mod utils;

#[derive(Clone, PartialEq, Properties)]
pub struct AppConfig {
    pub user: String,
    pub repo: String,
    pub branch: String,
}

#[wasm_bindgen]
pub fn run_app(user: String, repo: String, branch: String) {
    let props = AppConfig { user, repo, branch };
    yew::Renderer::<app::App>::with_props(props).render();
}
