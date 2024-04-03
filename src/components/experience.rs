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
    pub softskills: Vec<AttrValue>,
    pub description: AttrValue,
}

#[derive(PartialEq)]
pub struct ExperienceData {
    pub jobs: Vec<Rc<ExperienceSingleData>>,
    pub schools: Vec<Rc<ExperienceSingleData>>,
    pub certifs: Vec<Rc<CertificationData>>,
    pub contests: Vec<Rc<ContestData>>,
}

#[derive(PartialEq)]
pub struct ContestData {
    pub name: AttrValue,
    pub link: AttrValue,
}

#[derive(PartialEq)]
pub struct CertificationData {
    pub name: AttrValue,
    pub url: AttrValue,
    pub image: AttrValue,
    pub text: AttrValue
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
            <div>
                <div class="infos">
                    <IconTitle title={props.data.role.clone()} icon="assets/img/role.svg" />
                </div>
                <div class="infos">
                    <IconTitle title={props.data.start.clone()} icon="assets/img/calendar.svg" />
                    <IconTitle title={props.data.duration.clone()} icon="assets/img/time.svg" />
                    <IconTitle title={props.data.location.clone()} icon="assets/img/location.svg" />
                </div>
                <div class="description">
                    {props.data.description.clone()}
                </div>
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
            <div class="elements">{elements}</div>
        </div>  
    }
}

#[derive(Properties, PartialEq)]
struct CertificationProps {
    data: Vec<Rc<CertificationData>>
}

#[function_component]
fn Certifications(props: &CertificationProps) -> Html {

    let elements: Html = props.data.clone().into_iter().map(|c| {
        html!{
            <a class="animate" href={c.url.clone()} target="_blank"><img alt={c.name.clone()} src={c.image.clone()} /></a>
        }
    }).collect();

    html!{
        <div class="listing">
            <IconTitle title={"Certifications"} icon={"assets/img/certif.svg"} />
            <div class="certifs">
                {elements}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ContestsProps {
    data: Vec<Rc<ContestData>>
}

#[function_component]
fn Contests(props: &ContestsProps) -> Html {
    let elements: Html = props.data.clone().into_iter().map(|c| html!{
        html!{
            <a target="_blank" class="animate" href={c.link.clone()}>{c.name.clone()}</a>
        }
    }).collect();

    html!{
        <div class="listing">
            <IconTitle title={"Contests"} icon={"assets/img/podium.svg"} />
            <div class="contests">
                {elements}
            </div>
        </div>
    }
}


#[function_component]
pub fn Experience(props: &ExperienceProps) -> Html{
    html!{
        <div class="experiences">
            <Listing name="Jobs" icon="assets/img/job.svg" list={props.data.jobs.clone()} />
            <Listing name="School" icon="assets/img/school.svg" list={props.data.schools.clone()} />
            <Certifications data={props.data.certifs.clone()} />
            <Contests data={props.data.contests.clone()} />
        </div>
    }
}