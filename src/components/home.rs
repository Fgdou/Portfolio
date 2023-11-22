use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct LogoProps {
    path: &'static str,
    name: &'static str,
}

#[derive(Properties, PartialEq)]
struct LogoListProps {
    title: &'static str,
    logos: Vec<LogoProps>,
}

#[function_component]
fn Logo(props: &LogoProps) -> Html {
    html!{
        <img class="logo" src={props.path} alt={props.name}/>
    }
}

#[function_component]
fn LogoList(props: &LogoListProps) -> Html{

    let logos: Html = props.logos.iter().map(move |logo| {
        html!{
            <Logo path={logo.path} name={logo.name} />
        }
    }).collect();

    html!{
        <div class="logo-list">
            <h3>{props.title}</h3>
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
            name: "Rust",
            path: "/assets/img/rust.svg"
        },
        LogoProps{
            name: "C++",
            path: "/assets/img/cpp.svg"
        },
        LogoProps{
            name: "Java",
            path: "/assets/img/java.svg"
        },
        LogoProps{
            name: "Typescript",
            path: "/assets/img/typescript.svg"
        },
    );

    let technos = vec!(
        LogoProps{
            name: "Linux",
            path: "/assets/img/linux.svg"
        },
        LogoProps{
            name: "NodeJS",
            path: "/assets/img/nodejs.svg"
        },
        LogoProps{
            name: "React",
            path: "/assets/img/react.svg"
        },
        LogoProps{
            name: "Angular",
            path: "/assets/img/angular.svg"
        },
        LogoProps{
            name: "AWS",
            path: "/assets/img/aws.svg"
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