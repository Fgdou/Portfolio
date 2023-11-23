use std::rc::Rc;

use yew::prelude::*;

#[derive(PartialEq)]
pub struct ExperienceSingleData {
    pub image: AttrValue,
    pub role: AttrValue,
    pub start: AttrValue,
    pub duration: AttrValue,
    pub location: AttrValue,
    pub skills: Vec<AttrValue>,
    pub softskills: Vec<AttrValue>
}

#[derive(PartialEq)]
pub struct ExperienceData {
    pub jobs: Vec<Rc<ExperienceSingleData>>,
    pub schools: Vec<Rc<ExperienceSingleData>>
}

#[derive(Properties, PartialEq)]
pub struct ExperienceProps {
    pub data: Rc<ExperienceData>
}
#[derive(Properties, PartialEq)]
pub struct ExperienceSingleProps {
    pub data: Rc<ExperienceSingleData>
}

#[derive(Properties, PartialEq)]
struct SkillsListProps {
    list: Vec<AttrValue>
}

#[function_component]
fn SkillsList(props: &SkillsListProps) -> Html {
    let elements: Html = props.list.clone().into_iter().map(|e| {
        html!{
            <li>{e}</li>
        }
    }).collect();
    html!{
        <ul class="skill-list">
            {elements}
        </ul>
    }
}

#[function_component]
fn ExperienceSingle(props: &ExperienceSingleProps) -> Html {
    html!{
        <div class="experience">
            <img src={props.data.image.clone()} />
            <div class="infos">
                <IconTitle title={props.data.role.clone()} icon="" />
                <IconTitle title={props.data.start.clone()} icon="" />
                <IconTitle title={props.data.duration.clone()} icon="" />
                <IconTitle title={props.data.location.clone()} icon="" />
            </div>
            <div class="split">
                <SkillsList list={props.data.skills.clone()} />
                <SkillsList list={props.data.softskills.clone()} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct IconTitleProps {
    icon: AttrValue,
    title: AttrValue
}
#[function_component]
fn IconTitle(props: &IconTitleProps) -> Html{
    html!{
        <div class="icon-title">
            <img src={props.icon.clone()}/>
            <span>{props.title.clone()}</span>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ListingProps {
    icon: AttrValue,
    name: AttrValue,
    list: Vec<Rc<ExperienceSingleData>>
}

#[function_component]
fn Listing(props: &ListingProps) -> Html {
    let elements: Html = props.list.clone().into_iter().map(|e| {
        html!{
            <ExperienceSingle data={e.clone()} />
        }
    }).collect();
    html!{
        <div class="listing">
            <IconTitle title={props.name.clone()} icon={props.icon.clone()} />
            {elements}
        </div>  
    }
}

#[function_component]
pub fn Experience(props: &ExperienceProps) -> Html{
    html!{
        <div class="experience">
            <Listing name="Jobs" icon="" list={props.data.jobs.clone()} />
            <Listing name="School" icon="" list={props.data.schools.clone()} />
        </div>
    }
}