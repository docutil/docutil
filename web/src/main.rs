use yew::{function_component, html};

mod md_component;
mod util;
use md_component::*;

#[function_component(WebRoot)]
fn web_root() -> Html {
    html! {
        <div class="root"><MdView src={"/test.md"}></MdView></div>
    }
}

fn main() {
    yew::start_app::<WebRoot>();
}
