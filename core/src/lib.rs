use config::Config;
use router::Router;
use sycamore::prelude::*;

mod components;
mod config;
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
pub fn main(config: &Config) {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    let root = config.get_root_path();
    let title = config.get_title();
    let footer_message = config.get_footer_message();

    let router = Router::new();
    let main_md = Signal::new(String::new());
    let sidebar_md = Signal::new(format!("{}{}", root, "SIDEBAR.md"));
    let update_route = Box::new(cloned!((router, main_md, root) => move || {
        let (path, _) = router.route().unwrap();
        let home_page = format!("{}{}", root, "README.md").replace("//", "/");
        let path = format!("{}{}",root, path).replace("//", "/");

        log::info!("home_page = {}", home_page);
        log::info!("path = {}", path);

        let path = if path == root.as_str() { home_page } else { path };
        main_md.set(path);
    }));

    update_route();
    on_popstate(update_route);

    sycamore::render(|| {
        view! {
            header {
                div {
                    a(href=root) {
                        (title)
                    }
                }
            }
            div(class="wrapper") {
                div(class="post-wrapper") {
                    article(class="post") {
                        Post(main_md.handle())
                    }
                    aside(class="aside") {
                        div(class="content-wrapper") {
                            Post(sidebar_md.handle())
                        }
                    }
                }
            }
            footer {
                div {
                    (footer_message)
                }
            }
        }
    });
}
