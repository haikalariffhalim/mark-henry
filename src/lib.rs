use wasm_bindgen::prelude::*;
use yew::prelude::*;

// 1. REGISTER THE MODULES (So Rust knows these files exist)
pub mod app;
pub mod parser;
pub mod types;
pub mod utils;

// 2. DEFINE CONFIG (Props for the App)
#[derive(Clone, PartialEq, Properties)]
pub struct AppConfig {
    pub user: String,
    pub repo: String,
    pub branch: String,
}

// 3. STARTUP FUNCTION (Called from JavaScript)
#[wasm_bindgen]
pub fn run_app(user: String, repo: String, branch: String) {
    let props = AppConfig { user, repo, branch };
    yew::Renderer::<app::App>::with_props(props).render();
}
