use sycamore::futures::spawn_local_in_scope;
use sycamore::prelude::*;

use crate::util::{highlight_all, load_md_contents, render_markdown};

#[component(MdRenderer<G>)]
fn md_renderer(doc: ReadSignal<String>) -> View<G> {
    let html = create_memo(cloned!(doc => move || {
        render_markdown((*doc.get()).as_str())
    }));

    create_effect(cloned!(doc => move || {
        let _ = doc.get();
        spawn_local_in_scope(async move {
            highlight_all();
        });
    }));

    view! {
        div(class="markdown-body", dangerously_set_inner_html=(*html.get()).as_str())
    }
}

#[component(MdView<G>)]
fn md_view(src: ReadSignal<String>) -> View<G> {
    let doc = Signal::new(String::from(""));

    create_effect(cloned!(doc => move || {
        let src = (*src.get()).clone();
        spawn_local_in_scope(cloned!(doc => async move {
            let text = load_md_contents(&src).await.unwrap_or_default();
            doc.set(text);
        }));
    }));

    view! {
        MdRenderer(doc.handle())
    }
}

#[component(Post<G>)]
pub fn post(md_src: ReadSignal<String>) -> View<G> {
    view! {
        MdView(md_src)
    }
}
