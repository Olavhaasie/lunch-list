#![recursion_limit = "256"]

mod app;
mod login;
mod models;
mod routes;

use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wee-alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}
