use crate::util::{highlight_all, load_md_contents, render_markdown};
use log::debug;
use sycamore::web::tags::*;
use sycamore::{futures::spawn_local, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[component(inline_props)]
pub fn PostInner(md: Signal<String>) -> View {
    debug!("PostInner doc = {}", md.get_clone());

    div()
        .class("markdown-body")
        .dangerously_set_inner_html(md.get_clone())
        .into()
}

#[component(inline_props)]
pub fn Post(md_src: Signal<String>) -> View {
    let doc = create_signal(String::from("loading ..."));
    let div_ref = create_node_ref();

    create_effect(move || {
        let url = md_src.get_clone();

        spawn_local(async move {
            let text = load_md_contents(&url).await.unwrap_or_default();
            let page = render_markdown(&text);
            doc.set(page.html.clone());
            debug!("update doc");

            // 在 view! 宏中似乎不能直接设置 innerHTML，手工设置
            div_ref
                .get()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .set_inner_html(&doc.get_clone());
        });
    });

    create_effect(move || {
        doc.track();
        debug!("track create_effect");

        spawn_local(async {
            highlight_all();
        });
    });

    view! {
        div(r#ref=div_ref, class="markdown-body")
    }
}
