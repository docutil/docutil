use std::vec;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::RequestMode;

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
) -> anyhow::Result<Vec<Hit>> {
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
