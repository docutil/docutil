use gloo::utils::{document, document_element, window};
use sycamore::{futures::ScopeSpawnFuture, prelude::*};
use wasm_bindgen::{prelude::*, JsCast};

use crate::config::Config;
use crate::router::Router;
use crate::search::*;
use crate::util::{highlight_all, load_md_contents, render_markdown, render_one_markdown};

#[component]
pub fn BackTop<G: Html>(ctx: ScopeRef) -> View<G> {
    let div_ref = ctx.create_node_ref();
    let default_classes = "back-top-wrapper rounded border p-1";
    let wrapper_classes = create_rc_signal(format!("{} hidden", default_classes));

    {
        let on_scroll: Box<dyn Fn()> = Box::new({
            let wrapper_classes = wrapper_classes.clone();
            move || {
                let scroll_top = document_element().scroll_top();
                log::debug!("on_scroll: {}", scroll_top);

                if scroll_top > 300 {
                    wrapper_classes.set(format!("{} show", default_classes));
                } else {
                    wrapper_classes.set(format!("{} hidden", default_classes));
                }
            }
        });

        let listener = Closure::wrap(on_scroll);
        document()
            .add_event_listener_with_callback("scroll", listener.as_ref().unchecked_ref())
            .unwrap_throw();
        listener.forget();
    }

    let scroll_top = {
        move || {
            log::debug!("scroll_top");
            document_element().set_scroll_top(0);
        }
    };

    view! {ctx,
        div(class=(*wrapper_classes.get()).clone(), title="回到顶部") {
            div(ref=div_ref, on:click=move |_| {scroll_top()}) {
                span(class="icon-3x icon-top")
            }
        }
    }
}

#[component]
pub fn Post<'a, G: Html>(ctx: ScopeRef<'a>, md_src: &'a ReadSignal<String>) -> View<G> {
    let doc = ctx.create_signal(String::from("loading ..."));

    ctx.create_effect(move || {
        let url = (*md_src.get()).clone();
        ctx.spawn_future(async move {
            let text = load_md_contents(&url).await.unwrap_or_default();
            let page = render_markdown(&text);
            doc.set(page.html.clone());
        });
    });

    ctx.create_effect(|| {
        doc.track();

        ctx.spawn_future(async {
            highlight_all();
        });
    });

    view! {ctx,
        div(class="markdown-body", dangerously_set_inner_html=&(*doc.get()))
    }
}

fn on_popstate(f: Box<dyn FnMut()>) {
    let closure = Closure::wrap(f);

    window()
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .unwrap_throw();

    closure.forget();
}

#[component]
pub fn App<G: Html>(ctx: ScopeRef, props: &Config) -> View<G> {
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

            log::debug!("home_page = {}", home_page);
            log::debug!("path = {}", path);

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

    // 切换文章后，回到顶部
    ctx.create_effect(|| {
        _main_md.track();

        ctx.spawn_future(async {
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
