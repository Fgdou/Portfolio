use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct LogoProps {
    path: AttrValue,
    name: AttrValue,
}

#[derive(Properties, PartialEq)]
struct LogoListProps {
    title: AttrValue,
    logos: Vec<LogoProps>,
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

#[function_component]
pub fn Home() -> Html {
    let languages = vec!(
        LogoProps{
            name: AttrValue::from("Rust"),
            path: AttrValue::from("/assets/img/rust.svg")
        },
        LogoProps{
            name: AttrValue::from("C++"),
            path: AttrValue::from("/assets/img/cpp.svg")
        },
        LogoProps{
            name: AttrValue::from("Java"),
            path: AttrValue::from("/assets/img/java.svg")
        },
        LogoProps{
            name: AttrValue::from("Typescript"),
            path: AttrValue::from("/assets/img/typescript.svg")
        },
    );

    let technos = vec!(
        LogoProps{
            name: AttrValue::from("Linux"),
            path: AttrValue::from("/assets/img/linux.svg"),
        },
        LogoProps{
            name: AttrValue::from("NodeJS"),
            path: AttrValue::from("/assets/img/nodejs.svg"),
        },
        LogoProps{
            name: AttrValue::from("React"),
            path: AttrValue::from("/assets/img/react.svg"),
        },
        LogoProps{
            name: AttrValue::from("Angular"),
            path: AttrValue::from("/assets/img/angular.svg"),
        },
        LogoProps{
            name: AttrValue::from("AWS"),
            path: AttrValue::from("/assets/img/aws.svg"),
        },
    );

    html! {
        <div class="home container">
            <div class="text">
                <div class="title">
                    <h1>{"Fabien GOARDOU"}</h1>
                    <h2>{"Software Engineer"}</h2>
                </div>
                <LogoList title="Languages" logos={languages}/>
                <LogoList title="Technologies" logos={technos}/>
            </div>
            <img class="profile-picture" src="/assets/img/profile.jpeg"/>
        </div>
    }
}