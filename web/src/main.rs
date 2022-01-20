use sycamore::prelude::*;

mod md_component;
mod util;
use md_component::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|| {
        let md_src = Signal::new(String::from("/test.md"));

        view! {
            div(class="root") {
                MdView(md_src.handle())
            }
        }
    });
}
