use std::rc::Rc;

use yew::prelude::*;

use crate::components::*;

pub struct Data {
    pub home_props: HomeProps,
    pub experiences: ExperienceData,
    pub projects: Vec<ProjectData>,
    pub contact: ContactData,
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
                        softskills: ["Micro Services", "Team Work", "Stand-up"].into_iter().map(AttrValue::from).collect(),
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
                ),
                certifs: vec!(
                    Rc::from(CertificationData{
                        name: AttrValue::from("TOEIC"),
                        url: AttrValue::from("https://www.etsglobal.org"),
                        image: AttrValue::from("assets/img/TOEIC.png"),
                        text: AttrValue::from("C1 980")
                    }),
                    Rc::from(CertificationData{
                        name: AttrValue::from("CLES"),
                        url: AttrValue::from("https://www.certification-cles.fr/"),
                        image: AttrValue::from("assets/img/CLES.png"),
                        text: AttrValue::from("B2")
                    }),
                    Rc::from(CertificationData{
                        name: AttrValue::from("OVHcloud Discover IaaS"),
                        url: AttrValue::from("https://partner.ovhcloud.training/badges/badge.php?hash=d5304a253dc3b4f08f0b147899eab9eb699e7653"),
                        image: AttrValue::from("assets/img/Discover_IaaS_Products.png"),
                        text: AttrValue::from("")
                    }),
                    Rc::from(CertificationData{
                        name: AttrValue::from("OVHcloud Discover Support IaaS"),
                        url: AttrValue::from("https://partner.ovhcloud.training/badges/badge.php?hash=b94fe1bae4c4014a2acc1fa69372c3c4c64bf2da"),
                        image: AttrValue::from("assets/img/Discover_Support_for_IaaS_Products.png"),
                        text: AttrValue::from("")
                    }),
                ),
                contests: vec!(
                    Rc::from(ContestData{
                        name: AttrValue::from("SWERC"),
                        link: AttrValue::from("https://swerc.eu"),
                    }),
                    Rc::from(ContestData{
                        name: AttrValue::from("AdventOfCode"),
                        link: AttrValue::from("https://adventofcode.com/"),
                    }),
                    Rc::from(ContestData{
                        name: AttrValue::from("BreakTheCode"),
                        link: AttrValue::from("https://jobs.soprasteria.com/details/2023/11/23/default-calendar/break-the-code---edition-ille-et-vilaine-morbihan"),
                    }),
                    Rc::from(ContestData{
                        name: AttrValue::from("BattleDev"),
                        link: AttrValue::from("https://www.linkedin.com/posts/esir-univ-rennes_la-battledev-de-thales-a-eu-lieu-mardi-dernier-activity-7119350861679443968-2uAt?utm_source=share&utm_medium=member_desktop"),
                    }),
                )
            },
            projects: vec!(
                ProjectData{
                    name: AttrValue::from("AdventOfCode"),
                    github_link: Some(AttrValue::from("https://github.com/Fgdou/AdventOfCode2023")),
                    demo_link: None,
                    description: AttrValue::from("In 2023 : resolving an algorithm based problem every day in Rust. This is a fun event before Christmas, and I participated every years since 2020."),
                    image: AttrValue::from("assets/img/projects/aoc.jpeg"),
                },
                ProjectData{
                    name: AttrValue::from("GameOfLife"),
                    github_link: Some(AttrValue::from("https://github.com/Fgdou/GameOfLife")),
                    demo_link: None,
                    description: AttrValue::from("This project is coded in C++ just using the terminal for output. It uses the ANSI escape codes for moving the cursor around."),
                    image: AttrValue::from("assets/img/projects/gameoflife.gif")
                },
                ProjectData{
                    name: AttrValue::from("AdventureGame"),
                    github_link: Some(AttrValue::from("https://github.com/Fgdou/ProjProg2022")),
                    demo_link: None,
                    description: AttrValue::from("During this 30 hour project, we built from scratch a game on the adventure theme. The goal was to propose a playable game in not much time, but working with a list of requirement. There are no libraries or code except the SDL2."),
                    image: AttrValue::from("assets/img/projects/adventuregame.png")
                },
                ProjectData{
                    name: AttrValue::from("MandelbrotJS"),
                    github_link: Some(AttrValue::from("https://github.com/Fgdou/Mandelbrot_js")),
                    demo_link: Some(AttrValue::from("https://fgdou.github.io/Mandelbrot_js/")),
                    description: AttrValue::from("This is a simple project in javascript. In can be seen on desktop or mobile."),
                    image: AttrValue::from("assets/img/projects/mandelbrot.gif")
                },
                ProjectData{
                    name: AttrValue::from("Tower Defence"),
                    github_link: Some(AttrValue::from("https://github.com/Fgdou/Warcraft_V2")),
                    demo_link: None,
                    description: AttrValue::from("This is a java school project, with the StdDraw library for the GUI."),
                    image: AttrValue::from("assets/img/projects/warcraft.gif")
                },
            ),
            contact: ContactData { 
                email: AttrValue::from("fabigoardou@gmail.com"), 
                linkedin: AttrValue::from("https://www.linkedin.com/in/fgdou/"), 
                github: AttrValue::from("https://www.github.com/Fgdou"), 
                instagram: AttrValue::from("https://www.instagram.com/Fgdou/"), 
            }
        }
    }
}
