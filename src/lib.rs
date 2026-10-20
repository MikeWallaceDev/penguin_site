#![recursion_limit = "256"]
pub mod app;
pub mod components;
#[cfg(feature = "ssr")]
pub mod database;
pub mod models;
pub mod notifications;
pub(crate) mod routes;
pub mod validation;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use components::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
