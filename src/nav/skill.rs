use dioxus::prelude::*;

use crate::data::TheWorld;

#[component]
pub fn Skill_Lab() -> Element {
    let the_world = use_context::<Signal<TheWorld>>();

    rsx! {
        h1 { "Skill Lab page Damn" }
    }
}

#[component]
pub fn SkillView(id: u32) -> Element {
    rsx! { "Skill: {id}" }
}
