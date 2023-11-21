use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            {"Hello world"}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
