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

fn is_abs_uri(uri: &str) -> bool {
    let _uri = uri.trim().to_lowercase();
    _uri.starts_with("http://") || _uri.starts_with("http://") || _uri.starts_with("//")
}

pub fn render_markdown(doc: &str) -> String {
    let mut parser = Parser::new_ext(doc, Options::all());

    let mut parsed = Vec::new();
    let mut get_heading = false;
    while let Some(event) = parser.next() {
        let _event = match event {
            Event::Start(Tag::Link(link_type, dest, title)) => {
                if is_abs_uri(&dest) {
                    Event::Start(Tag::Link(link_type, dest, title)).into()
                } else {
                    let rewired = format!("/#/{}", dest).replace("//", "/");
                    Event::Start(Tag::Link(link_type, CowStr::from(rewired), title)).into()
                }
            }
            Event::Start(Tag::Heading(level, id, class_list)) => {
                get_heading = true;

                Event::Start(Tag::Heading(level, id, class_list))
            }
            Event::Text(text) => {
                if get_heading {
                    log::debug!("get heading: {}", text);
                }

                Event::Text(text)
            }
            _ => event,
        };

        parsed.push(_event);
    }

    let mut output = String::new();
    html::push_html(&mut output, parsed.into_iter());

    output
}
