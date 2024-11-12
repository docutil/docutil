use std::{fmt::Display, rc::Rc, vec};

use gloo::{net::http::Request, utils::document};
use log::warn;
use serde::{Deserialize, Serialize};
use sycamore::{futures::spawn_local, prelude::*, Props};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{KeyboardEvent, MouseEvent, RequestMode};

use crate::config::APP_OPTIONS;

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hit {
    pub line: String,
    pub path: String,
}

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Props, Clone)]
pub struct SearchResultDialogProps {
    pub keyword: String,
    pub list: Vec<Hit>,
    pub on_close: Rc<Box<dyn Fn()>>,
}

#[component]
fn SearchResultDialog(props: SearchResultDialogProps) -> View {
    let on_clone_from_props = props.on_close.clone();
    let on_close = move |_: MouseEvent| on_clone_from_props();
    let search_result = create_memo(move || props.list.clone());

    view! {
        div(class="search-result-dialog modal lg:bg-slate-700 lg:bg-opacity-10") {
            div(class="modal-card bg-white lg:rounded-md lg:shadow-md") {
                div(class="modal-card-head p-2 border-0 border-b border-gray-300") {
                    p(class="modal-card-title") {
                        (format!("搜索：{}", props.keyword.clone()))
                    }
                    button(class="icon-3x icon-close", on:click=on_close.clone())
                }
                div(class="modal-card-body p-4 markdown-body") {
                    ul {
                        Indexed (
                            list=search_result,
                            view=move |it| view! {
                                li {
                                    a(href=format!("/#/{}",it.path), on:click=on_close.clone()) {
                                        (it.line)
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
            move || {
                view! {
                    (SearchResultDialog(props.clone()))
                }
            },
            &el,
        );
    }
    document().body().unwrap().append_child(&el).unwrap_throw();
}

#[component]
pub fn SearchBox() -> View {
    let keyword = create_signal(String::new());

    let start_search = move |event: KeyboardEvent| {
        if event.key_code() != 13 {
            return;
        }

        let word = keyword.get_clone();
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
    };

    view! {
         div(class="search-box") {
            input(placeholder="搜索 ...",
                bind:value=keyword,
                on:keypress=start_search,
                class="shadow rounded px-2 py-1 border-none w-full",
                r#type="search") {}
        }
    }
}
