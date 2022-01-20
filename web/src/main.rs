use sycamore::prelude::*;

mod components;
mod router;
mod util;
use components::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    let route = router::get_route().unwrap_or_default();
    let md_src = Signal::new(route.doc.unwrap_or("testdata/README.md".to_string()));

    sycamore::render(|| {
        view! {
            MarkNote(md_src.handle())
        }
    });
}
