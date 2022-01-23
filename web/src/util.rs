use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag};
use reqwasm::http::Request;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs, js_name = highlightAll )]
    pub fn highlight_all();
}

pub async fn load_md_contents(url: &str) -> anyhow::Result<String> {
    let resp = Request::get(url).send().await?;
    let text = resp.text().await?;
    Ok(text)
}

pub fn render_markdown(doc: &str) -> String {
    let parser = Parser::new_ext(doc, Options::all());

    let parser = parser.map(|event| match event {
        Event::Start(Tag::Link(link_type, dest, title)) => {
            log::info!("tag::link {:?}, {}, {}", link_type, dest, title);

            let dest_new = format!("/#/{}", dest).replace("//", "/");
            let dest = CowStr::from(dest_new);
            Event::Start(Tag::Link(link_type, dest, title)).into()
        }
        _ => event,
    });

    let mut output = String::new();
    html::push_html(&mut output, parser);

    output
}
