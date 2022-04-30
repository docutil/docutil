use gloo::utils::{document, document_element};
use sycamore::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

#[component]
pub fn BackTop<G: Html>(ctx: Scope) -> View<G> {
    let div_ref = create_node_ref(ctx);
    let default_classes = "back-top-wrapper rounded border p-1";
    let wrapper_classes = create_rc_signal(format!("{} hidden", default_classes));

    {
        let on_scroll: Box<dyn Fn()> = Box::new({
            let wrapper_classes = wrapper_classes.clone();
            move || {
                let scroll_top = document_element().scroll_top();
                log::debug!("on_scroll: {}", scroll_top);

                if scroll_top > 300 {
                    wrapper_classes.set(format!("{} show", default_classes));
                } else {
                    wrapper_classes.set(format!("{} hidden", default_classes));
                }
            }
        });

        let listener = Closure::wrap(on_scroll);
        document()
            .add_event_listener_with_callback("scroll", listener.as_ref().unchecked_ref())
            .unwrap_throw();
        listener.forget();
    }

    let scroll_top = {
        move || {
            log::debug!("scroll_top");
            document_element().set_scroll_top(0);
        }
    };

    view! {ctx,
        div(class=(*wrapper_classes.get()).clone(), title="回到顶部") {
            div(ref=div_ref, on:click=move |_| {scroll_top()}) {
                span(class="icon-3x icon-top")
            }
        }
    }
}
