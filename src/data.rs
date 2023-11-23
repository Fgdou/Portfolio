use std::rc::Rc;

use yew::prelude::*;

use crate::components::*;

pub struct Data {
    pub home_props: HomeProps,
    pub experiences: ExperienceData
}

impl Default for Data {
    fn default() -> Self {
        Self { 
            home_props: HomeProps{
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
            },
            experiences: ExperienceData{
                jobs: vec!(
                    Rc::from(ExperienceSingleData{
                        image: AttrValue::from("assets/img/amazon.svg"),
                        role: AttrValue::from("Software Developer Engineer"),
                        start: AttrValue::from("June 2023"),
                        duration: AttrValue::from("4 Months"),
                        location: AttrValue::from("Cambridge, UK"),
                        skills: ["AI", "Python", "HuggingFace", "AWS"].into_iter().map(AttrValue::from).collect(),
                        softskills: ["Team Work", "SCRUM", "Communication"].into_iter().map(AttrValue::from).collect(),
                    }),
                    Rc::from(ExperienceSingleData{
                        image: AttrValue::from("assets/img/uiris.png"),
                        role: AttrValue::from("Software Developer Intern"),
                        start: AttrValue::from("June 2022"),
                        duration: AttrValue::from("3 Months"),
                        location: AttrValue::from("Carquefou, FR"),
                        skills: ["Java Springboot", "Jenkins", "VueJS", "SQL"].into_iter().map(AttrValue::from).collect(),
                        softskills: ["Micro Services", "Team Work", "Stand-ip"].into_iter().map(AttrValue::from).collect(),
                    }),
                ),
                schools: vec!(
                    Rc::from(ExperienceSingleData{
                        image: AttrValue::from("assets/img/esir.png"),
                        role: AttrValue::from("Software Engineer Master"),
                        start: AttrValue::from("Sept. 2019"),
                        duration: AttrValue::from("5 Years"),
                        location: AttrValue::from("Rennes, FR"),
                        skills: ["Java", "C", "DevOps"].into_iter().map(AttrValue::from).collect(),
                        softskills: ["Engineering", "Communication", "Soft-Skills"].into_iter().map(AttrValue::from).collect(),
                    }),
                )
            }
        }
    }
}