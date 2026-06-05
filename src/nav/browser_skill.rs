use bevy_ecs::{
    name::Name,
    query::{With, Without},
};
use convert_case::ccase;
use dioxus::prelude::*;
use dioxus_primitives::ContentSide;
use strum::IntoEnumIterator;

use crate::{components::*, data::*};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    pub id: SkillID,
    pub img: String,
    pub name: String,
    pub creator: String,
    pub creator_id: u32,
    pub tags: Vec<Skill>,
    pub rating: u32,
    pub total_review: usize,
    pub description: String,
}

#[derive(Clone, Debug, Default)]
pub struct FilterState {
    sort_by: SortBy,
    item_tags: Vec<Skill>,
    creator_name: String,
    skill_name: String,
    show: u8,
    page: usize,
}

impl FilterState {
    pub fn new() -> Self {
        Self {
            show: 15,
            ..Default::default()
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    strum::EnumCount,
    strum::EnumIter,
    strum::Display,
    strum::AsRefStr,
)]
pub enum SortBy {
    ReviewLowHigh,
    ReviewHighLow,
    RatingLowHigh,
    RatingHighLow,
    Id,
    #[default]
    None,
}

impl SortBy {
    const fn emoji(&self) -> &'static str {
        match self {
            SortBy::ReviewLowHigh => "📈",
            SortBy::ReviewHighLow => "📉",
            SortBy::RatingLowHigh => "📈",
            SortBy::RatingHighLow => "📉",
            SortBy::Id => "🆔",
            SortBy::None => "❌",
        }
    }
}

/// BrowseSkill page
#[component]
pub fn BrowseSkill() -> Element {
    let mut state = use_context::<Signal<FilterState>>();

    rsx! {

        div {
            class: "center",
            align_items: "flex-start",
            align_self: "center",
            justify_self: "center",
            flex_direction: "column",
            margin_bottom: "1rem",
            max_width: "400px",
            p { font_size: "0.8rem",
                "Due to technical issues, you need to manually type at the url to find user profile and skill. Example:"
            }

            p { font_size: "0.8rem", "'Write program (2)' is '/skill_lab/2'" }

            p { font_size: "0.8rem", "'Sam (1)' is '/profile/1'" }

        }

        div { flex_direction: "column",

            div { id: "filter_1",

                Sort_By_Tag {}

                a { "Creator:" }
                Input {
                    oninput: move |e: FormEvent| state.write().creator_name = e.value(),
                    placeholder: "Creator Name",
                    value: "{state().creator_name}",
                }

                a { "Skill:" }
                Input {
                    oninput: move |e: FormEvent| state.write().skill_name = e.value(),
                    placeholder: "Skill Name",
                    value: "{state().skill_name}",
                }

                Skill_Tag_Filter {}

            }

            div {
                class: "center",
                margin_top: "1rem",
                align_items: "flex-start",
                flex_wrap: "wrap",

                div { id: "showcase", class: "center", flex_grow: 1, Output_skill {} }

            }
        }
    }
}

#[component]
fn Skill_Tag_Filter() -> Element {
    let mut state = use_context::<Signal<FilterState>>();

    let items = Skill::iter().enumerate().map(|(i, it)| {
        let cased = ccase!(sentence, it.as_ref());
        rsx! {
            SelectOption::<Skill> { index: i, value: it, text_value: "{it}", "{it.emoji()} {cased}" }
        }
    });

    rsx! {
        a { "Tags:" }
        SelectMulti::<Skill> {
            default_values: vec![],
            width: "16rem",
            on_values_change: move |event: Vec<Skill>| {
                state.write().item_tags = event.clone();
            },
            aria_placeholder: "Hi",
            SelectGroup { {items} }
        }
    }
}

#[component]
fn Sort_By_Tag() -> Element {
    let mut state = use_context::<Signal<FilterState>>();

    let sort_by = SortBy::iter().enumerate().map(|(i, s)| {
        let cased = ccase!(sentence , s.as_ref());
        rsx! {
            SelectOption::<Option<SortBy>> { index: i, value: s, text_value: "{cased}", "{s.emoji()} {cased}" }
        }
    });

    rsx! {
        a { "Sort By: " }
        Select::<Option<SortBy>> {
            width: "14rem",
            default_value: Some(SortBy::default()),
            on_value_change: move |event: Option<Option<SortBy>>| {
                state.write().sort_by = event.flatten().unwrap();
            },
            SelectGroup { style: "width: 13.5rem", {sort_by} }
        }
    }
}

#[component]
fn Output_skill() -> Element {
    let state = use_context::<Signal<FilterState>>();
    let the_world = use_context::<Signal<TheWorld>>();

    let output: Memo<Vec<Item>> = use_memo(move || {
        let filt_state = state();
        let world = the_world.read();
        let mut new_item: Vec<Item> = Vec::new();

        let Some(mut q_name) = world.try_query_filtered::<(&Name, &UserID), With<User>>() else {
            return Vec::new();
        };
        let mut q_review = world.try_query_filtered::<&Rating, Without<UnReview>>();

        let Some(mut q_skill) = world.try_query::<(
            &SkillID,
            &Name,
            &SkillImage,
            &SkillCreatedBy,
            &SkillTag,
            &SkillReviewFrom,
            &Description,
        )>() else {
            return Vec::new();
        };

        let q_skill_iter = q_skill.iter(&world);

        for (id, name, img, creator, tag, reviewed_from, description) in q_skill_iter {
            let has_name = filt_state.skill_name.is_empty();
            if !has_name {
                if !name
                    .to_lowercase()
                    .contains(&filt_state.skill_name.to_lowercase())
                {
                    continue;
                }
            }

            let (creator_name, creator_id) = q_name.get(&world, creator.0).unwrap();
            let has_creator_name = creator_name
                .to_lowercase()
                .contains(&filt_state.creator_name.to_lowercase());
            if !has_creator_name {
                continue;
            }

            let has_all_tag = filt_state
                .item_tags
                .iter()
                .all(|need_tag| tag.contains(need_tag));
            if !has_all_tag {
                continue;
            }

            let tags = tag.iter().cloned().collect::<Vec<_>>();

            let (total_review, rating) = if let Some(ref mut q_review) = q_review {
                let a = reviewed_from
                    .iter()
                    .map_while(|&ent| q_review.get(&world, ent).ok())
                    .collect::<Vec<_>>();
                let total: u32 = a.iter().map(|b| b.0 as u32).sum();
                (a.len(), total)
            } else {
                (0, 0)
            };

            let item = Item {
                id: id.clone(),
                img: img.0.to_owned(),
                name: name.to_string(),
                creator: creator_name.to_string(),
                creator_id: creator_id.0,
                tags,
                rating,
                total_review,
                description: description.0.clone(),
            };

            new_item.push(item);
        }

        match filt_state.sort_by {
            SortBy::ReviewLowHigh => new_item.sort_by(|a, b| a.total_review.cmp(&b.total_review)),
            SortBy::ReviewHighLow => new_item.sort_by(|a, b| b.total_review.cmp(&a.total_review)),
            SortBy::RatingLowHigh => {
                new_item.sort_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap())
            }
            SortBy::RatingHighLow => {
                new_item.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap())
            }
            SortBy::Id => new_item.sort_by_key(|a| a.id),
            SortBy::None => {}
        }

        new_item
    });

    rsx! {
        for item in output.iter() {
            div { flex_direction: "column", class: "contained",

                Tooltip {
                    TooltipTrigger {
                        img {
                            class: "active",
                            object_fit: "contain",
                            width: "100%",
                            height: "120px",
                            border_radius: "1rem",
                            align_self: "center",
                            src: if item.img.is_empty() { "https://github.com/bevyengine/bevy/blob/main/assets/branding/icon.png?raw=true"
                                .to_string() } else { item.img.clone() },
                        }
                    }
                    TooltipContent { side: ContentSide::Top, style: "width: 200px",
                        p {
                            {
                                let all_sort: String = item
                                    .tags
                                    .iter()
                                    .map(|i| ccase!(sentence, i.as_ref()))
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                format!("{}", all_sort)
                            }
                        }
                    }
                }

                h1 { margin_bottom: "0.8rem", "{item.name} ({item.id.0})" }
                // Link {
                //     text_decoration: "none",
                //     to: Route::SkillView { id: item.id.0 },
                //     "{item.name}"
                // }
                p { word_wrap: "break-word",
                    {
                        let closest = item.description.ceil_char_boundary(28);
                        let mut first = item.description.clone();
                        let split = first.split_off(closest);
                        if !split.is_empty() {
                            first.push_str("...");
                        }
                        first
                    }
                }
                // Link {
                //     font_size: "0.8rem",
                //     text_decoration: "none",
                //     to: Route::SkillView {
                //         id: item.creator_id,
                //     },
                //     "{item.creator}"
                // }
                p { font_size: "0.8rem", "{item.creator} ({item.creator_id})" }

                p { font_size: "0.8rem", "Rating: {item.rating} | Review: {item.total_review}" }
            }
        }
    }
}
