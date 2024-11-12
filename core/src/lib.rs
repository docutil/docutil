// extern crate alloc;
use config::APP_OPTIONS;
use gloo::utils::{document_element, window};
use sycamore::{futures::spawn_local, prelude::*};
use wasm_bindgen::{prelude::*, JsCast};

mod components;
mod config;
mod router;
mod util;
use crate::components::{BackTop, Post, SearchBox};
use crate::{config::Config, router::Router, util::render_one_markdown};

#[cfg(target_arch = "wasm32")]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> = LockedAllocator::new(FreeListAllocator::new());

fn on_popstate(f: Box<dyn FnMut()>) {
    let closure = Closure::wrap(f);

    window()
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .unwrap_throw();

    closure.forget();
}

#[component]
pub fn App() -> View {
    let app_options = APP_OPTIONS.get().unwrap();

    let root = app_options.get_root_path();
    let title = app_options.get_title();
    let footer_message = render_one_markdown(&app_options.get_footer_message());
    let enable_search = app_options.is_enable_search();

    let main_md_url = create_signal(String::new());
    let sidebar_md_url = create_signal(format!("{}{}", root, "SIDEBAR.md"));

    {
        let router = Router::new();

        let root = app_options.get_root_path();
        let update_route = Box::new(move || {
            let (path, _) = router.route().unwrap_throw();
            let home_page = format!("{}{}", root, "README.md").replace("//", "/");
            let path = format!("{}{}", root, path).replace("//", "/");

            let path = if path == root.as_str() { home_page } else { path };
            main_md_url.set(path);
        });

        update_route();
        on_popstate(update_route);
    }

    // 切换文章后，回到顶部
    {
        create_effect(move || {
            main_md_url.track();

            spawn_local(async {
                document_element().set_scroll_top(0);
            });
        });
    }

    view! {
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
                        Post(md_src=main_md_url)
                    }
                    aside(class="column aside shadow lg:shadow-none") {
                        div(class="content-wrapper p-4") {
                            (if enable_search {
                                view!(div(class="mb-4") {
                                    SearchBox()
                                })
                            } else {
                                view!()
                            })
                            div {
                                Post(md_src=sidebar_md_url)
                            }
                        }
                    }
                }
            }
        }
        footer(class="mb-4 pb-12") {
            div(class="container px-4") {
                div(dangerously_set_inner_html=footer_message)
            }
        }
        BackTop()
    }
}

#[wasm_bindgen(js_name = initApp)]
pub fn init_app(config: Config) {
    console_error_panic_hook::set_once();

    #[cfg(not(debug_assertions))]
    console_log::init_with_level(log::Level::Error).unwrap();

    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Trace).unwrap();

    let _ = APP_OPTIONS.set(config);

    sycamore::render(|| {
        view! {
            // 按文档说明，un-cloak 属性用于解决初始化 uno 前的空白问题。
            // 文档：https://github.com/unocss/unocss/tree/main/packages/runtime
            main(un-cloak="true") {
                App()
            }
        }
    });
}
