use gloo::utils::{document, document_element, window};
use sycamore::{futures::ScopeSpawnFuture, prelude::*};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Event, KeyboardEvent};

use crate::config::Config;
use crate::router::Router;
use crate::search::*;
use crate::util::{highlight_all, load_md_contents, render_markdown, render_one_markdown};

#[component]
pub fn SearchDialog<G: Html>(ctx: ScopeRef) -> View<G> {
    let keyword = ctx.create_signal(String::new());
    let search_result = ctx.create_signal(vec![]);
    let dialog_classes = ctx.create_signal(String::from("search-result-dialog hidden"));

    let search = {
        move |event: Event| {
            let text = (*keyword.get()).clone();
            if text.is_empty() {
                return;
            }

            let event = event.dyn_into::<KeyboardEvent>().unwrap();
            if event.key_code() == 13 {
                let search_result = search_result.clone();
                ctx.spawn_future(async move {
                    let result = remote_search(&text, 1, 100).await.unwrap();
                    log::info!("remote_search result is: {:?}", result);
                    search_result.set(result);
                    dialog_classes.set(String::from("modal search-result-dialog show"))
                });
            }
        }
    };

    let close = {
        let dialog_classes = dialog_classes.clone();
        let search_result = search_result.clone();
        let keyword = keyword.clone();
        move |_: Event| {
            dialog_classes.set(String::from("modal search-result-dialog hidden"));
            search_result.set(vec![]);
            keyword.set(String::new())
        }
    };

    view! {ctx,
        div(class="search-box") {
            input(bind:value=keyword, on:keypress=search, placeholder="搜索 ...", class="input is-rounded", type="search")
        }
        div(class=dialog_classes) {
            div(class="modal-background", on:click=close)
            div(class="modal-card mt-4") {
                div(class="modal-card-head p-3") {
                    p(class="modal-card-title") { "搜索结果" }
                    button(class="delete", aria-label="close", on:click=close)
                }
                div(class="modal-card-body markdown-body") {
                    h2 { "搜索结果" }
                    ul {
                        Indexed {
                            iterable: search_result,
                            view: move |ctx, it| view! {ctx,
                                li(class="box") {
                                    a(href=format!("/#/{}",it.path), on:click=close) {
                                        (it.line)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BackTop<G: Html>(ctx: ScopeRef) -> View<G> {
    let div_ref = ctx.create_node_ref();
    let wrapper_classes = create_rc_signal(String::from("back-top-wrapper hidden"));

    {
        let on_scroll: Box<dyn Fn()> = Box::new({
            let wrapper_classes = wrapper_classes.clone();
            move || {
                let scroll_top = document_element().scroll_top();
                log::debug!("on_scroll: {}", scroll_top);

                if scroll_top > 300 {
                    wrapper_classes.set(String::from("back-top-wrapper show"));
                } else {
                    wrapper_classes.set(String::from("back-top-wrapper hidden"));
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
                span(class="icon icon-top")
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
        header(class="mb-4") {
            div(class="container") {
                div(class="columns is-align-content-space-between is-align-items-center") {
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
                    article(class="column is-3-4 box post") {
                        Post(_main_md)
                    }
                    aside(class="column aside") {
                        div(class="content-wrapper") {
                            div(class="mb-4") {
                                SearchDialog()
                            }
                            div {
                                Post(_sidebar_md)
                            }
                        }
                    }
                }
            }
        }
        footer(class="mb-4") {
            div(class="container pl-2 pr-2") {
                div(dangerously_set_inner_html=&footer_message)
            }
        }
        BackTop()
    }
}
