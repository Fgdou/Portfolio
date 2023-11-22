use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct HeaderButtonProps {
    #[prop_or_default]
    selected: bool,
    name: AttrValue,
    #[prop_or_default]
    callback: Option<Callback<AttrValue>>,
}

#[function_component]
fn HeaderButton(props: &HeaderButtonProps) -> Html {

    let name = props.name.clone();

    let onclick = match props.callback.clone() {
        Some(callback) => {
            Callback::from(move |_| {
                callback.emit(name.to_owned());
            })
        },
        None => Callback::from(|_|{})
    };

    html! {
        <div {onclick} class={if props.selected {"header-btn active"} else {"header-btn"}}>
            {props.name.clone()}
        </div>
    }
}

#[function_component]
pub fn Header() -> Html {
    let buttons = [
        AttrValue::from("Home"), 
        AttrValue::from("Experience"),
        AttrValue::from("Project"),
        AttrValue::from("Contact")
        ];
    let active = use_state(|| buttons[0].clone());

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