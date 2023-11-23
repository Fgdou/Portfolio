use std::rc::Rc;

use yew::prelude::*;

#[derive(PartialEq)]
pub struct ExperienceSingleData {
    pub image: AttrValue,
    pub role: AttrValue,
    pub start: AttrValue,
    pub duration: AttrValue,
    pub location: AttrValue,
    pub skills: Vec<AttrValue>
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

#[function_component]
fn ExperienceSingle(props: &ExperienceSingleProps) -> Html {
    html!{

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
    html!{
        <div class="listing">
            <h1 class="header"><img class="icon" src={props.icon.clone()}/>{props.name.clone()}</h1>
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