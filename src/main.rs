mod components;
use yew::prelude::*;

use crate::components::{Header, Home};

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Header/>
            <Home/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
