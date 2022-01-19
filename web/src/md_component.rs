use reqwasm::http::Request;
use web_sys::HtmlElement;
use yew::{function_component, html, use_effect_with_deps, use_node_ref, use_state, Properties, use_effect};

use crate::util::{render_markdown, highlight_all};

#[derive(Properties, PartialEq)]
pub struct MdRendererProps {
    pub contents: String,
}

#[function_component(MdRenderer)]
fn md_renderer(props: &MdRendererProps) -> Html {
    let div_ref = use_node_ref();

    if let Some(div) = div_ref.cast::<HtmlElement>() {
        let post_html = render_markdown(&props.contents);
        div.set_inner_html(&post_html);
    }

    html! {
        <div class="md-renderer markdown-body" ref={div_ref}></div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MdViewProps {
    pub src: String,
}

#[function_component(MdView)]
pub fn md_view(props: &MdViewProps) -> Html {
    let url = &props.src;
    let contents = use_state(|| String::new());

    {
        let contents = contents.clone();
        let url = url.clone();
        use_effect_with_deps(
            move |_| {
                let contents = contents.clone();
                let url = url.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let doc = Request::get(&url).send().await.unwrap().text().await.unwrap();

                    contents.set(doc);
                });

                || ()
            },
            (),
        );
    }

    {
        use_effect(|| {
            highlight_all();

            || ()
        })
    }

    let _contents = (*contents).clone();
    html! {
        <div class="md-view">
            <MdRenderer contents={(*contents).clone()}></MdRenderer>
        </div>
    }
}
