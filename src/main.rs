mod components;
use yew::prelude::*;

use crate::components::Header;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Header/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
