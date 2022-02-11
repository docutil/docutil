use config::Config;
use router::Router;
use sycamore::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

mod components;
mod config;
mod router;
mod util;
use components::*;

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

#[component]
fn App<G: Html>(ctx: ScopeRef, props: &Config) -> View<G> {
    let root = props.get_root_path();
    let title = props.get_title();
    let footer_message = props.get_footer_message();

    let main_md = create_rc_signal(String::new());
    let sidebar_md = create_rc_signal(format!("{}{}", root, "SIDEBAR.md"));

    {
        let router = Router::new();
        
        let main_md = main_md.clone();
        let root = props.get_root_path();
        let update_route = Box::new(move || {
            let (path, _) = router.route().unwrap();
            let home_page = format!("{}{}", root, "README.md").replace("//", "/");
            let path = format!("{}{}", root, path).replace("//", "/");

            log::info!("home_page = {}", home_page);
            log::info!("path = {}", path);

            let path = if path == root.as_str() {
                home_page
            } else {
                path
            };
            main_md.set(path);
        });

        update_route();
        on_popstate(update_route);
    }

    let _main_md = {
        let main_md = main_md.clone();
        ctx.create_memo(move || (*main_md.get()).clone())
    };
    let _sidebar_md = {
        let sidebar_md = sidebar_md.clone();
        ctx.create_memo(move || (*sidebar_md.get()).clone())
    };

    view! {ctx,
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
                    Post(_main_md)
                }
                aside(class="aside") {
                    div(class="content-wrapper") {
                        Post(_sidebar_md)
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
}

#[wasm_bindgen]
pub fn main(config: &Config) {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|ctx| {
        view! {ctx,
            App(config)
        }
    });
}
