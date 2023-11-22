use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct HeaderButtonProps {
    #[prop_or_default]
    selected: bool,
    name: &'static str,
    #[prop_or_default]
    callback: Option<Callback<&'static str>>,
}

#[function_component]
fn HeaderButton(props: &HeaderButtonProps) -> Html {

    let name = props.name;

    let onclick = match props.callback.clone() {
        Some(callback) => {
            Callback::from(move |_| {
                callback.emit(name);
            })
        },
        None => Callback::from(|_|{})
    };

    html! {
        <div {onclick} class={if props.selected {"header-btn active"} else {"header-btn"}}>
            {props.name}
        </div>
    }
}

#[function_component]
pub fn Header() -> Html {
    let buttons = ["Home", "Experience", "Project", "Contact"];
    let active = use_state(|| buttons[0]);

    let elements: Html = buttons.iter().map(|name| {
        let active = active.clone();
        html! {
            <HeaderButton {name} selected={*name == *active} callback={move |s| active.set(s)} />
        }
    }).collect();

    html! {
        <header>
            {elements}
        </header>
    }
}