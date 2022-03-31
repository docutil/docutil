use sycamore::{futures::ScopeSpawnLocal, prelude::*};
use crate::util::{highlight_all, load_md_contents, render_markdown};

#[component]
pub fn Post<'a, G: Html>(ctx: ScopeRef<'a>, md_src: &'a ReadSignal<String>) -> View<G> {
    let doc = ctx.create_signal(String::from("loading ..."));

    ctx.create_effect(move || {
        let url = (*md_src.get()).clone();
        ctx.spawn_local (async move {
            let text = load_md_contents(&url).await.unwrap_or_default();
            let page = render_markdown(&text);
            doc.set(page.html.clone());
        });
    });

    ctx.create_effect(|| {
        doc.track();

        ctx.spawn_local(async {
            highlight_all();
        });
    });

    view! {ctx,
        div(class="markdown-body", dangerously_set_inner_html=&(*doc.get()))
    }
}
