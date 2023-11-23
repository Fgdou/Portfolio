mod components;
use std::rc::Rc;

use yew::{prelude::*, props};

use crate::components::*;

#[function_component]
fn App() -> Html {

    let root = props!(
        HomeProps{
            fonction: AttrValue::from("Software Engineer"),
            name: AttrValue::from("Fabien GOARDOU"),
            logo_list: vec!(
                LogoListProps{
                    logos: vec!(
                        LogoProps{
                            name: AttrValue::from("Rust"),
                            path: AttrValue::from("assets/img/rust.svg")
                        },
                        LogoProps{
                            name: AttrValue::from("C++"),
                            path: AttrValue::from("assets/img/cpp.svg")
                        },
                        LogoProps{
                            name: AttrValue::from("Java"),
                            path: AttrValue::from("assets/img/java.svg")
                        },
                        LogoProps{
                            name: AttrValue::from("Typescript"),
                            path: AttrValue::from("assets/img/typescript.svg")
                        },
                    ),
                    title: AttrValue::from("Languages")
                },
                LogoListProps{
                    logos: vec!(
                        LogoProps{
                            name: AttrValue::from("Linux"),
                            path: AttrValue::from("assets/img/linux.svg"),
                        },
                        LogoProps{
                            name: AttrValue::from("NodeJS"),
                            path: AttrValue::from("assets/img/nodejs.svg"),
                        },
                        LogoProps{
                            name: AttrValue::from("React"),
                            path: AttrValue::from("assets/img/react.svg"),
                        },
                        LogoProps{
                            name: AttrValue::from("Angular"),
                            path: AttrValue::from("assets/img/angular.svg"),
                        },
                        LogoProps{
                            name: AttrValue::from("AWS"),
                            path: AttrValue::from("assets/img/aws.svg"),
                        },
                    ),
                    title: AttrValue::from("Techologies")
                },
            )
        }
    );

    let experiences = Rc::from(ExperienceData{
        jobs: vec!(
            Rc::from(ExperienceSingleData{
                image: AttrValue::from("assets/img/amazon.svg"),
                role: AttrValue::from("Software Developer Engineer"),
                start: AttrValue::from("June 2023"),
                duration: AttrValue::from("4 Months"),
                location: AttrValue::from("Cambridge, UK"),
                skills: ["AI", "Python", "HuggingFace", "AWS"].into_iter().map(AttrValue::from).collect(),
                softskills: ["Team Work", "SCRUM", "Communication"].into_iter().map(AttrValue::from).collect(),
            })
        ),
        schools: vec!(

        )
    });

    html! {
        <div>
            <Header/>
            <Home ..root />
            <Experience data={experiences} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
