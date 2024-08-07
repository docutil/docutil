use std::{fmt::Display, rc::Rc, vec};

use gloo::{net::http::Request, utils::document};
use log::warn;
use serde::{Deserialize, Serialize};
use sycamore::{futures::spawn_local, prelude::*};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, KeyboardEvent, RequestMode};

use crate::config::APP_OPTIONS;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchApiQueryParam {
    pub keyword: String,
    pub page_index: u32,
    pub page_size: u32,
}

impl Display for SearchApiQueryParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pairs = vec![
            ("keyword", self.keyword.clone()),
            ("pageIndex", self.page_index.to_string()),
            ("pageSize", self.page_size.to_string()),
        ];
        write!(f, "{}", serde_urlencoded::to_string(pairs).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hit {
    pub line: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    let query_params = SearchApiQueryParam {
        keyword: keyword.into(),
        page_index,
        page_size,
    };

    let endpoint = APP_OPTIONS.get().unwrap().get_search_api_endpoint();

    let endpoint = format!("{endpoint}?{query_params}");
    let req = Request::get(&endpoint).mode(RequestMode::Cors).send().await?;

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
    pub keyword: String,
    pub list: Vec<Hit>,
    pub on_close: Rc<Box<dyn Fn()>>,
}

#[component]
fn SearchResultDialog<'a, G: Html>(ctx: Scope<'a>, props: SearchResultDialogProps) -> View<G> {
    let on_close = Rc::new(move |_: Event| (props.on_close)());
    let search_result = create_memo(ctx, move || props.list.clone());

    let btn_on_click = on_close.clone();

    view! {ctx,
        div(class="search-result-dialog modal lg:bg-slate-700 lg:bg-opacity-10") {
            div(class="modal-card bg-white lg:rounded-md lg:shadow-md") {
                div(class="modal-card-head p-2 border-0 border-b border-gray-300") {
                    p(class="modal-card-title") {
                        (format!("搜索：{}", props.keyword.clone()))
                    }
                    button(class="icon-3x icon-close", on:click=move |e| btn_on_click(e))
                }
                div(class="modal-card-body p-4 markdown-body") {
                    ul {
                        Indexed (
                            iterable=search_result,
                            view={
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
                        )
                    }
                }
            }
        }
    }
}

fn open_dialog<'a>(search_result: Vec<Hit>, keyword: &str) {
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
            keyword: keyword.into(),
            list: search_result,
            on_close: Rc::new(Box::new(on_close)),
        };

        sycamore::render_to(
            {
                let props = props.clone();
                move |ctx: Scope| {
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
pub fn SearchBox<G: Html>(ctx: Scope) -> View<G> {
    let keyword = create_signal(ctx, String::new());

    let start_search = {
        move |event: Event| {
            let event = event.dyn_into::<KeyboardEvent>().unwrap();
            if event.key_code() != 13 {
                return;
            }

            let word = (*keyword.get()).clone();
            if word.is_empty() {
                return;
            }

            spawn_local(async move {
                let res = remote_search(&word, 1, 100).await;
                match res {
                    Ok(result) => {
                        open_dialog(result, &word);
                        set_overflow(true);
                    }
                    Err(err) => {
                        warn!("unable to search, {}", err)
                    }
                }
            });

            keyword.set(String::new());
        }
    };

    view! {ctx,
         div(class="search-box") {
            input(bind:value=keyword,
                on:keypress=start_search,
                placeholder="搜索 ...",
                class="shadow rounded px-2 py-1 border-none w-full",
                type="search")
        }
    }
}
