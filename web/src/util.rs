use pulldown_cmark::{Options, Parser, html};
use wasm_bindgen::prelude::wasm_bindgen;


pub fn render_markdown(doc: &str) -> String {
    let parser = Parser::new_ext(doc, Options::all());

    let mut html_str = String::new();
    html::push_html(&mut html_str, parser);

    html_str
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs, js_name = highlightAll )]
    pub fn highlight_all();
}