use router::Router;
use sycamore::prelude::*;

mod components;
mod router;
mod util;
use components::*;
use wasm_bindgen::{prelude::Closure, JsCast};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn on_popstate(f: Box<dyn FnMut()>) {
    let closure = Closure::wrap(f);

    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .unwrap();

    log::info!("on_popstate");
    closure.forget();
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    let router = Router::new();

    let md_src = Signal::new(String::from("testdata/README.md"));
    on_popstate(Box::new(cloned!((router, md_src) => move || {
        let (path, search_params) = router.route().unwrap();

        let doc = search_params.doc.unwrap_or(String::from("testdata/README.md"));
        log::info!("[router] path: {}, doc: {}", path, doc);

        md_src.set(doc);
    })));

    sycamore::render(|| {
        view! {
            MarkNote(md_src.handle())
        }
    });
}
