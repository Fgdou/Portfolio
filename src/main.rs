mod components;
mod data;
use std::rc::Rc;

use yew::prelude::*;

use components::*;
use data::*;

#[function_component]
fn App() -> Html {

    let data = Data::default();

    let experiences = Rc::from(data.experiences);

    html! {
        <div>
            <Header/>
            <Home ..data.home_props />
            <Experience data={experiences} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
