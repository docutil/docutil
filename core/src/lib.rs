use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

mod components;
mod config;
mod router;
mod util;
mod search;
use components::*;
use config::Config;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main(config: &Config) {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).unwrap();
    
    sycamore::render(|ctx| {
        view! {ctx,
            App(config)
        }
    });
}
