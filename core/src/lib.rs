use gloo::utils::{document_element, window};
use log::debug;
use sycamore::futures::spawn_local;
use sycamore::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

mod components;
mod config;
mod router;
mod util;
use crate::{components::*, config::Config, router::Router, util::render_one_markdown};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn on_popstate(f: Box<dyn FnMut()>) {
    let closure = Closure::wrap(f);

    window()
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .unwrap_throw();

    closure.forget();
}

#[component]
pub fn App<G: Html>(ctx: Scope, props: &Config) -> View<G> {
    let root = props.get_root_path();
    let title = props.get_title();
    let footer_message = render_one_markdown(&props.get_footer_message());

    let main_md = create_rc_signal(String::new());
    let sidebar_md = create_rc_signal(format!("{}{}", root, "SIDEBAR.md"));

    {
        let router = Router::new();

        let main_md = main_md.clone();
        let root = props.get_root_path();
        let update_route = Box::new(move || {
            let (path, _) = router.route().unwrap_throw();
            let home_page = format!("{}{}", root, "README.md").replace("//", "/");
            let path = format!("{}{}", root, path).replace("//", "/");

            debug!("home_page = {}", home_page);
            debug!("path = {}", path);

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
        create_memo(ctx, move || (*main_md.get()).clone())
    };
    let _sidebar_md = {
        let sidebar_md = sidebar_md.clone();
        create_memo(ctx, move || (*sidebar_md.get()).clone())
    };

    // 切换文章后，回到顶部
    create_effect(ctx, || {
        _main_md.track();

        spawn_local(async {
            document_element().set_scroll_top(0);
        });
    });

    view! {ctx,
        header(class="lg:mb-4 mb-0") {
            div(class="container p-2") {
                div(class="columns justify-center items-center") {
                    div(class="column m-0 title") {
                        a(href=root) {
                            (title)
                        }
                    }
                    div(class="column m-0 quick-links")
                }
            }
        }
        section(class="mb-4") {
            div(class="container") {
                div(class="columns") {
                    article(class="column post is-3-4 shadow lg:rounded bg-white p-8") {
                        Post(_main_md)
                    }
                    aside(class="column aside shadow lg:shadow-none") {
                        div(class="content-wrapper p-4") {
                            div(class="mb-4") {
                                SearchBox()
                            }
                            div {
                                Post(_sidebar_md)
                            }
                        }
                    }
                }
            }
        }
        footer(class="mb-4 pb-12") {
            div(class="container px-4") {
                div(dangerously_set_inner_html=&footer_message)
            }
        }
        BackTop()
    }
}

#[wasm_bindgen(js_name = initApp)]
pub fn init_app(config: &Config) {
    console_error_panic_hook::set_once();

    #[cfg(not(debug_assertions))]
    console_log::init_with_level(log::Level::Info).unwrap();

    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Trace).unwrap();

    sycamore::render(|ctx| {
        view! {ctx,
            // 按文档说明，un-cloak 属性用于解决初始化 uno 前的空白问题。
            // 文档：https://github.com/unocss/unocss/tree/main/packages/runtime
            div(un-cloak=true) {
                App(config)
            }
        }
    });
}
