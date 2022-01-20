use pulldown_cmark::{html, Options, Parser};
use reqwasm::http::Request;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs, js_name = highlightAll )]
    pub fn highlight_all();
}

pub fn render_markdown(doc: &str) -> String {
    let parser = Parser::new_ext(doc, Options::all());

    let mut html_str = String::new();
    html::push_html(&mut html_str, parser);

    html_str
}

pub async fn load_md_contents(url: &str) -> anyhow::Result<String> {
    let resp = Request::get(url).send().await?;
    let text = resp.text().await?;
    Ok(text)
}
