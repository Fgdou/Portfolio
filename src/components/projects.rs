use std::rc::Rc;

use yew::prelude::*;

#[derive(PartialEq)]
pub struct ProjectData {
    pub name: AttrValue,
    pub github_link: Option<AttrValue>,
    pub demo_link: Option<AttrValue>,
    pub description: AttrValue,
    pub image: AttrValue,
}

#[derive(Properties, PartialEq)]
pub struct ProjectsProps {
    pub data: Rc<Vec<ProjectData>>
}

#[function_component]
pub fn Projects(props: &ProjectsProps) -> Html {
    let elements: Html = props.data.iter().map(|d| {
        let github: Html = match &d.github_link {
            Some(link) => html!{<a target="_blank" href={link.clone()}><img src="assets/img/github.svg"/></a>},
            None => html!{}
        };
        let link: Html = match &d.demo_link {
            Some(link) => html!{<a target="_blank" href={link.clone()}><img src="assets/img/link.svg"/></a>},
            None => html!{}
        };

        html!{
            <div class="project">
                <div class="text">
                    <div class="title">
                    <span>{d.name.clone()}</span>
                    <span>
                    {github}
                    {link}
                    </span>
                    </div>
                    <p class="description">{d.description.clone()}</p>
                </div>
                <img src={d.image.clone()} alt={d.name.clone()} />
            </div>
        }
    }).collect();

    html!{
        <div class="projects">
            {elements}
        </div>
    }
}