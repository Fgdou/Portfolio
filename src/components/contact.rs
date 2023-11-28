use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    icon: AttrValue,
    text: AttrValue,
    url: AttrValue,
}

#[function_component]
fn Icon(props: &IconProps) -> Html {
    html!{
        <div class="icon">
            <a class="animate" target="_blank" href={props.url.clone()}>
                <img src={props.icon.clone()}/>
                <span>{props.text.trim_start_matches("https://www.")}</span>
            </a>
        </div>
    }
}

#[derive(PartialEq)]
pub struct ContactData {
    pub email: AttrValue,
    pub linkedin: AttrValue,
    pub github: AttrValue,
    pub instagram: AttrValue
}

#[derive(Properties, PartialEq)]
pub struct ContactProps {
    pub data: ContactData
}

#[function_component]
pub fn Contact(props: &ContactProps) -> Html {
    html!{
        <div class="contacts">
            <div>
                <div class="header">{"Contact"}</div>
            <Icon icon={"assets/img/email.svg"} text={props.data.email.clone()} url={format!("mailto:{}", props.data.email)} />
            <Icon icon={"assets/img/linkedin.svg"} text={props.data.linkedin.clone()} url={props.data.linkedin.clone()} />
            <Icon icon={"assets/img/github.svg"} text={props.data.github.clone()} url={format!("mailto:{}", props.data.github.clone())} />
            <Icon icon={"assets/img/insta.svg"} text={props.data.instagram.clone()} url={format!("mailto:{}", props.data.instagram.clone())} />
            </div>
        </div>
    }
}