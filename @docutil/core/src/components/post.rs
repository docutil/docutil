use crate::util::{highlight_all, load_md_contents, render_markdown};
use sycamore::{futures::spawn_local, prelude::*};

#[component]
pub fn Post<'a, G: Html>(ctx: Scope<'a>, md_src: &'a ReadSignal<String>) -> View<G> {
    let doc = create_rc_signal(String::from("loading ..."));
    let _doc = create_signal(ctx, (*doc.get()).clone());

    {
        let doc = doc.clone();
        create_effect(ctx, move || {
            let url = (*md_src.get()).clone();

            let doc = doc.clone();
            spawn_local(async move {
                let text = load_md_contents(&url).await.unwrap_or_default();
                let page = render_markdown(&text);
                doc.set(page.html.clone());
            });
        });
    }

    {
        let doc = doc.clone();
        create_effect(ctx, move || {
            _doc.set((*doc.get()).clone());

            spawn_local(async {
                highlight_all();
            });
        });
    }

    view! {ctx,
        div(class="markdown-body", dangerously_set_inner_html=&(_doc.get()))
    }
}
