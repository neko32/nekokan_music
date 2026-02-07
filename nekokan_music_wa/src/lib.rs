mod api;
mod app;
mod form;
mod types;
mod validation;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    yew::Renderer::<app::App>::with_root(
        gloo_utils::document().get_element_by_id("app").unwrap(),
    )
    .render();
}
