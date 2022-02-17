use sycamore::{futures::ScopeSpawnFuture, prelude::*};
use wasm_bindgen::{prelude::*, JsCast};

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
        move || {
            let text = (*keyword.get()).clone();
            if text.is_empty() {
                return;
            }

            let search_result = search_result.clone();
            ctx.spawn_future(async move {
                let result = remote_search(&text, 1, 100).await.unwrap();
                log::info!("remote_search result is: {:?}", result);
                search_result.set(result);
                dialog_classes.set(String::from("search-result-dialog show"))
            });
        }
    };

    let close = {
        let dialog_classes = dialog_classes.clone();
        let search_result = search_result.clone();
        let keyword = keyword.clone();
        move || {
            dialog_classes.set(String::from("search-result-dialog hidden"));
            search_result.set(vec![]);
            keyword.set(String::new())
        }
    };

    view! {ctx,
        div(class="search-box") {
            input(bind:value=keyword)
            button(on:click=move |_| {search()}) {
                "🔎"
            }
        }
        div(class=dialog_classes) {
            div(class="search-result") {
                div(class="title") {
                    div { "搜索结果" }
                    div {
                        button(on:click=move |_| close()) {
                            "❌"
                        }
                    }
                }
                div(class="body") {
                    ul {
                        Indexed {
                            iterable: search_result,
                            view: |ctx, it| view! {ctx,
                                li {
                                    a(href=format!("/#/{}",it.path)) {
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

    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();

    {
        let on_scroll: Box<dyn Fn()> = Box::new({
            let wrapper_classes = wrapper_classes.clone();
            let document = document.clone();
            move || {
                let document_element = document.document_element().unwrap_throw();
                let scroll_top = document_element.scroll_top();
                log::info!("on_scroll: {}", scroll_top);

                if scroll_top > 300 {
                    wrapper_classes.set(String::from("back-top-wrapper show"));
                } else {
                    wrapper_classes.set(String::from("back-top-wrapper hidden"));
                }
            }
        });

        let listener = Closure::wrap(on_scroll);
        document
            .add_event_listener_with_callback("scroll", listener.as_ref().unchecked_ref())
            .unwrap_throw();
        listener.forget();
    }

    let scroll_top = {
        let document = document.clone();
        move || {
            log::debug!("scroll_top");
            let document_element = document.document_element().unwrap_throw();
            document_element.set_scroll_top(0);
        }
    };

    view! {ctx,
        div(class=(*wrapper_classes.get()).clone()) {
            div(ref=div_ref, on:click=move |_| {scroll_top()}) {
                span(class="icon-top") {
                    "△"
                }
            }
        }
    }
}

#[component]
pub fn Post<'a, G: Html>(ctx: ScopeRef<'a>, md_src: &'a ReadSignal<String>) -> View<G> {
    let doc = ctx.create_signal(String::from(""));

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

    web_sys::window()
        .unwrap_throw()
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .unwrap_throw();

    log::debug!("on_popstate");
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
            let window = web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let document_element = document.document_element().unwrap_throw();

            document_element.set_scroll_top(0);
        });
    });

    view! {ctx,
        header {
            div {
                div(class="title") {
                    a(href=root) {
                        (title)
                    }
                }
                div(class="quick-links")
            }
        }
        div(class="wrapper") {
            div(class="post-wrapper") {
                article(class="post") {
                    Post(_main_md)
                }
                aside(class="aside") {
                    div {
                        SearchDialog()
                    }
                    div(class="content-wrapper") {
                        Post(_sidebar_md)
                    }
                }
            }
        }
        footer {
            div(dangerously_set_inner_html=&footer_message)
        }
        BackTop()
    }
}
