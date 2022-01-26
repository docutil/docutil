use router::Router;
use sycamore::prelude::*;

mod components;
mod router;
mod util;
use components::*;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn on_popstate(f: Box<dyn FnMut()>) {
    let closure = Closure::wrap(f);

    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .unwrap();

    log::debug!("on_popstate");
    closure.forget();
}

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).unwrap();

    let router = Router::new();
    let main_md = Signal::new(String::new());
    let sidebar_md = Signal::new(String::from("SIDEBAR.md"));
    let update_route = Box::new(cloned!((router, main_md) => move || {
        let (path, _) = router.route().unwrap();
        let path = if path == "/" { String::from("/README.md") } else { path };
        main_md.set(path);
    }));

    update_route();
    on_popstate(update_route);

    sycamore::render(|| {
        view! {
            header {
                div {
                    a(href="/") {
                        "hello, world"
                    }
                }
            }
            div(class="wrapper") {
                div(class="post-wrapper") {
                    article(class="post") {
                        Post(main_md.handle())
                    }
                    sidebar(class="sidebar") {
                        div(class="content-wrapper") {
                            Post(sidebar_md.handle())
                        }
                    }
                }
            }
            footer {
                div {
                    a(href="https://beian.miit.gov.cn/") {
                        "ICP备2021172595号"
                    }
                }
            }
        }
    });
}
