use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LogoProps {
    pub path: AttrValue,
    pub name: AttrValue,
}

#[derive(Properties, PartialEq, Clone)]
pub struct LogoListProps {
    pub title: AttrValue,
    pub logos: Vec<LogoProps>,
}

#[function_component]
fn Logo(props: &LogoProps) -> Html {
    html!{
        <div class="logo-container">
            <img class="logo" src={props.path.clone()} alt={props.name.clone()}/>
            <span class="text">{props.name.clone()}</span>
        </div>
    }
}

#[function_component]
fn LogoList(props: &LogoListProps) -> Html{

    let logos: Html = props.logos.iter().map(move |logo| {
        html!{
            <Logo path={logo.path.clone()} name={logo.name.clone()} />
        }
    }).collect();

    html!{
        <div class="logo-list">
            <h3>{props.title.clone()}</h3>
            <div>
                {logos}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub name: AttrValue,
    pub fonction: AttrValue,
    #[prop_or_default]
    pub logo_list: Vec<LogoListProps>,
}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {

    let lists: Html = props.logo_list.clone().into_iter().map(move |l| {
        html! {
            <LogoList title={l.title.clone()} logos={l.logos} />
        }
    }).collect();

    html! {
        <div class="home container">
            <div class="text">
                <div class="title">
                    <h1>{props.name.clone()}</h1>
                    <h2>{props.fonction.clone()}</h2>
                </div>
                {lists}
            </div>
            <img class="profile-picture" src="/assets/img/profile.jpeg"/>
        </div>
    }
}