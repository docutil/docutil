use std::sync::atomic::{AtomicU32, Ordering};

use gloo::net::http::Request;
use pulldown_cmark::{html, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs, js_name = highlightAll )]
    pub fn highlight_all();
}

pub async fn load_md_contents(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = Request::get(url).send().await?;
    let text = resp.text().await?;
    Ok(text)
}

fn is_abs_uri(uri: &str) -> bool {
    let _uri = uri.trim().to_lowercase();
    _uri.starts_with("https://") || _uri.starts_with("http://") || _uri.starts_with("//")
}

#[derive(Clone, PartialEq, Eq)]
pub struct Outline {
    pub name: String,
    pub level: HeadingLevel,
    pub anchor: String,
}

#[derive(Clone, PartialEq, Eq)]
pub struct MarkdownPage {
    pub html: String,
    pub outlines: Vec<Outline>,
}

pub fn render_markdown(doc: &str) -> MarkdownPage {
    let mut outlines: Vec<Outline> = vec![];
    let mut some_heading: Option<Outline> = None;

    let header_id = AtomicU32::new(1);

    let parser = Parser::new_ext(doc, Options::all()).filter_map(|event| match event {
        // 处理 md 文件中的相对路径
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            if is_abs_uri(&dest_url) {
                Some(Event::Start(Tag::Link {
                    link_type: link_type,
                    dest_url: dest_url,
                    title: title,
                    id: id,
                }))
            } else {
                let rewired = format!("/#/{}", dest_url).replace("//", "/");
                Some(Event::Start(Tag::Link {
                    link_type: link_type,
                    dest_url: rewired.into(),
                    title: title,
                    id: id,
                }))
            }
        }
        Event::Start(Tag::Heading {
            level,
            id: _,
            classes: _,
            attrs: _,
        }) => {
            some_heading = Some(Outline {
                anchor: String::new(),
                level,
                name: String::new(),
            });
            None
        }
        Event::End(TagEnd::Heading(_level)) => {
            if some_heading.is_some() {
                let outline = some_heading.take().unwrap();
                let anchor = format!("{}_{}", outline.level, header_id.fetch_add(1, Ordering::Relaxed));

                outlines.push(Outline {
                    anchor: anchor.clone(),
                    level: outline.level,
                    name: outline.name.clone(),
                });

                Some(Event::Html(CowStr::from(format!(
                    "<{level} id=\"{anchor}\">{name}</{level}>",
                    anchor = anchor,
                    level = outline.level,
                    name = outline.name
                ))))
            } else {
                Some(event)
            }
        }
        Event::Text(ref text) => {
            if some_heading.is_some() {
                let mut outline = some_heading.take().unwrap();
                outline.name = format!("{}{}", outline.name, text.clone().to_string());

                some_heading = Some(outline);
                None
            } else {
                Some(event)
            }
        }
        Event::Code(ref code) => {
            if some_heading.is_some() {
                let mut outline = some_heading.take().unwrap();
                outline.name = format!("{}{}", outline.name, code.clone().to_string());

                some_heading = Some(outline);
                None
            } else {
                Some(event)
            }
        }
        _ => Some(event),
    });

    let mut output = String::new();
    html::push_html(&mut output, parser);

    MarkdownPage {
        html: output,
        outlines,
    }
}

pub fn render_one_markdown(doc: &str) -> String {
    render_markdown(doc).html
}
