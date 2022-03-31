use std::{rc::Rc, vec};

use gloo::utils::document;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::{futures::ScopeSpawnLocal, prelude::*};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, KeyboardEvent, RequestMode};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hit {
    pub line: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResult {
    pub hits: Vec<Hit>,
    pub limit: u32,
    pub offset: u32,

    #[serde(rename = "nbHits")]
    pub nb_hits: u64,
}

pub async fn remote_search(
    keyword: &str,
    page_index: u32,
    page_size: u32,
) -> Result<Vec<Hit>, Box<dyn std::error::Error>> {
    let base_url = "https://mn-search.lambdadriver.space/api/v1/yuekcc/search";
    // TODO uri encode 处理
    let url = format!("{base_url}?keyword={keyword}&pageIndex={page_index}&pageSize={page_size}");

    let req = Request::new(&url).mode(RequestMode::Cors).send().await?;
    let result = req.json::<SearchResult>().await;
    if result.is_ok() {
        Ok(result.unwrap().hits)
    } else {
        Ok(vec![])
    }
}

fn set_overflow(hidden: bool) {
    let body = document().body().unwrap();
    let result = if hidden {
        body.class_list().add_1("overflow-hidden")
    } else {
        body.class_list().remove_1("overflow-hidden")
    };

    result.unwrap_throw();
}

#[derive(Prop, Clone)]
pub struct SearchResultDialogProps {
    pub list: Vec<Hit>,
    pub on_close: Rc<Box<dyn Fn()>>,
}

#[component]
fn SearchResultDialog<'a, G: Html>(ctx: ScopeRef<'a>, props: SearchResultDialogProps) -> View<G> {
    let on_close = Rc::new(move |_: Event| (props.on_close)());
    let search_result = ctx.create_memo(move || props.list.clone());

    let btn_on_click = on_close.clone();

    view! {ctx,
        div(class="search-result-dialog modal lg:bg-slate-700 lg:bg-opacity-10") {
            div(class="modal-card bg-white lg:rounded-md lg:shadow-md") {
                div(class="modal-card-head p-2 border-0 border-b") {
                    p(class="modal-card-title") { "搜索结果" }
                    button(class="icon-3x icon-close", on:click=move |e| btn_on_click(e))
                }
                div(class="modal-card-body p-4 markdown-body") {
                    ul {
                        Indexed {
                            iterable: search_result,
                            view: {
                                let on_close = on_close.clone();
                                move |ctx, it| {
                                    let on_item_click = on_close.clone();
                                    view! {ctx,
                                        li {
                                            a(href=format!("/#/{}",it.path), on:click=move |e| on_item_click(e)) {
                                                (it.line)
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

fn open_dialog<'a>(search_result: Vec<Hit>) {
    let el = document().create_element("div").unwrap_throw();
    let on_close = {
        let el = el.clone();
        move || {
            el.remove();
            set_overflow(false);
        }
    };

    {
        let props = SearchResultDialogProps {
            list: search_result,
            on_close: Rc::new(Box::new(on_close)),
        };

        sycamore::render_to(
            {
                let props = props.clone();
                move |ctx: &Scope| {
                    view! {ctx,
                        SearchResultDialog(props)
                    }
                }
            },
            &el,
        );
    }
    document().body().unwrap().append_child(&el).unwrap_throw();
}

#[component]
pub fn SearchBox<G: Html>(ctx: ScopeRef<'_>) -> View<G> {
    let keyword = ctx.create_signal(String::new());

    let reset = {
        let keyword = keyword.clone();
        move || {
            keyword.set(String::new());
        }
    };

    let search = {
        move |event: Event| {
            let text = (*keyword.get()).clone();
            if text.is_empty() {
                return;
            }

            let event = event.dyn_into::<KeyboardEvent>().unwrap();
            if event.key_code() == 13 {
                ctx.spawn_local(async move {
                    let result = remote_search(&text, 1, 100).await.unwrap();
                    open_dialog(result);
                    reset();
                    set_overflow(true);
                });
            }
        }
    };

    view! {ctx,
        div(class="search-box") {
            input(bind:value=keyword,
                on:keypress=search,
                placeholder="搜索 ...",
                class="shadow rounded px-2 py-1 border-none w-full",
                type="search")
        }
    }
}
