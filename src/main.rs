mod components;
mod data;
use std::rc::Rc;

use yew::prelude::*;

use components::*;
use data::*;

#[function_component]
fn App() -> Html {

    let page = use_state(|| AttrValue::from("home"));

    let data = Data::default();

    let experiences = Rc::from(data.experiences);

    html! {
        <div id="home">
            <Header page={page.clone()} />
            <div class="container">
                <div id={"Home"} onmouseenter={
                    let page = page.clone();
                    move |_| page.set(AttrValue::from("Home"))
                }><Home ..data.home_props /></div>
                <div id={"Experience"} onmouseenter={
                    let page = page.clone();
                    move |_| page.set(AttrValue::from("Experience"))
                }><Experience data={experiences} /></div>
                <div id={"Projects"} onmouseenter={
                    let page = page.clone();
                    move |_| page.set(AttrValue::from("Projects"))
                }><Projects /></div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
