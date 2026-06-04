use std::collections::HashSet;

use crate::{components::*, data::*};
use bevy_ecs::prelude::*;
use convert_case::ccase;
use dioxus::prelude::*;
use strum::IntoEnumIterator as _;

use crate::{
    components::{Button, Input, Select, SelectMulti},
    data::{CurrentUser, Skill, SkillCreatedBy, SkillMark, SkillTag, TheWorld},
};

#[derive(Default, Debug, Clone)]
enum CreateState {
    Successful,
    NameAlreadyUsed,
    RequireFull,
    #[default]
    None,
}

#[derive(Default, Debug, Clone)]
struct SetKnow {
    tag: Vec<Skill>,
    state: CreateState,
}

impl CreateState {
    pub fn color(&self) -> &str {
        match self {
            CreateState::Successful => "blue",
            CreateState::NameAlreadyUsed => "red",
            CreateState::RequireFull => "red",
            CreateState::None => "",
        }
    }

    pub fn text(&self) -> &str {
        match self {
            CreateState::Successful => "Successful created",
            CreateState::NameAlreadyUsed => "Skill name already used",
            CreateState::RequireFull => "Require name and atleast 1 tag",
            CreateState::None => "",
        }
    }
}

#[component]
pub fn CreateSkill() -> Element {
    let mut the_world = use_context::<Signal<TheWorld>>();
    let mut name = use_signal(String::new);
    let mut img_link = use_signal(String::new);
    let mut set_know = use_context_provider(|| Signal::new(SetKnow::default()));

    if !the_world.read().has_current_user() {
        return rsx! {
            h1 { "Require login" }
        };
    }

    let create_skill = move |_| {
        let mut r_name = name.write();
        let mut set_know = set_know.write();
        let mut world = the_world.write();

        let creator = world
            .query_filtered::<Entity, With<CurrentUser>>()
            .single(&world)
            .unwrap();

        if r_name.is_empty() || set_know.tag.is_empty() {
            set_know.state = CreateState::RequireFull;
            return;
        }

        let skill_name = Name::new(r_name.to_string());
        let used_name = world
            .try_query_filtered::<&Name, With<SkillMark>>()
            .map(|mut q| q.iter(&world).any(|nam| nam.eq(&skill_name)))
            .unwrap_or_default();
        if used_name {
            set_know.state = CreateState::NameAlreadyUsed;
            return;
        }

        let hash = set_know.tag.drain(..).collect::<HashSet<Skill>>();
        world.spawn((
            SkillMark,
            SkillTag(hash),
            skill_name,
            SkillCreatedBy(creator),
        ));
        world.flush();

        set_know.state = CreateState::Successful;
        r_name.clear();
    };

    rsx! {
        Tagging {}
        Input {
            oninput: move |e: FormEvent| name.set(e.value()),
            placeholder: "Enter title for the skill name",
            value: name,
        }

        Input {
            oninput: move |e: FormEvent| img_link.set(e.value()),
            placeholder: "Enter image link for the skill banner",
            value: img_link,
        }

        Button { onclick: create_skill, "Create" }

    }
}

#[component]
fn Tagging() -> Element {
    let mut set_know = use_context::<Signal<SetKnow>>();

    let items = Skill::iter().enumerate().map(|(i, it)| {
        let cased = ccase!(title, it.as_ref());
        rsx! {
            SelectOption::<Skill> { index: i, value: it, text_value: "{it}", "{it.emoji()} {cased}" }
        }
    });

    rsx! {
        p { margin_top: "1rem", "Skill Tags (Multi)" }
        SelectMulti::<Skill> {
            default_values: vec![],
            width: "16rem",
            on_values_change: move |event: Vec<Skill>| {
                set_know.write().tag = event.clone();
            },
            SelectGroup { {items} }
        }

        // Select::<Option<Skill>> {
        //     width: "14rem",
        //     default_value: None,
        //     on_value_change: move |event: Option<Option<Skill>>| {
        //         set_know.write().tag = event.flatten().map(|v| vec![v]).unwrap_or_default();
        //     },
        //     SelectGroup { style: "width: 13.5rem", {sort_by} }
        // }
        p { color: set_know().state.color(), "{set_know().state.text()}" }
    }
}
