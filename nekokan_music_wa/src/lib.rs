mod api;
mod app;
mod form;
mod types;
mod validation;

use wasm_bindgen::prelude::*;

/// タブタイトル・メイン見出し用。`Cargo.toml` の `version` をビルド時に埋め込む。
pub const APP_TITLE_WITH_VERSION: &str = concat!("Nekokan Music ", env!("CARGO_PKG_VERSION"));

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    gloo_utils::document().set_title(APP_TITLE_WITH_VERSION);
    yew::Renderer::<app::App>::with_root(
        gloo_utils::document().get_element_by_id("app").unwrap(),
    )
    .render();
}
