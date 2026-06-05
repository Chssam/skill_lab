use bevy_ecs::prelude::*;
use dioxus::prelude::*;

use crate::{data::*, nav::Route};

#[component]
pub fn SkillView(id: u32) -> Element {
    let the_world = use_context::<Signal<TheWorld>>();

    let me_name = use_memo(move || {
        let world = the_world.read();
        let id = id;

        let op_q_user = world.try_query_filtered::<(&Name, &UserID), With<User>>();

        let v = world
            .try_query_filtered::<(&Name, &SkillID, &SkillCreatedBy, &Description, &SkillImage), With<SkillMark>>()
            .map(|mut q| {
                q.iter(&world).find_map(|(name, q_id, op_skill_create, description, img)| {
                    q_id.0
                        .eq(&id)
                        .then(|| (name.to_string(), q_id.0, op_skill_create.clone(), description.0.clone(), img.clone()))
                })
            })
            .flatten();

        let out = v.map(|v_1| {
            let ye = op_q_user
                .map(|mut q_s| {
                    let ab = q_s
                        .get(&world, *v_1.2)
                        .ok()
                        .map(|a| (a.0.to_string(), a.1.0));
                    ab
                })
                .unwrap_or_default();

            (v_1.0, v_1.1, ye, v_1.3, v_1.4)
        });

        out
    });

    rsx! {
        div { class: "center", flex_direction: "column",

            if let Some((skill_name, skill_id, created_by, description, img)) = me_name() {
                div {
                    class: "contained",
                    display: "flex",
                    flex_grow: 1,
                    flex_shrink: 0,
                    max_width: "100vh",
                    flex_wrap: "wrap",
                    flex_direction: "row",

                    div { margin_right: "1rem",

                        p { "Name: {skill_name} ({skill_id})" }
                        if let Some((name, id)) = created_by {
                            div {
                                Link { to: Route::ProfileView { id: id }, "Created by: {name}" }
                            }
                        }
                        p { "Description:" }
                        p { word_break: "break-word", "{description}" }

                    }
                    img {
                        class: "active",
                        border_radius: "1rem",
                        max_width: "300px",
                        src: img.0,
                    }
                }

                div {
                    class: "center",
                    display: "flex",
                    flex_wrap: "wrap",
                    flex_direction: "row",

                    div { class: "contained",
                        h1 { "Stats:" }
                        p { "Attended by: 0" }
                        p { "Reviewed by: 0" }
                    }

                    div { class: "contained",
                        h1 { "Reviewed by:" }
                    }
                }
            } else {
                div { class: "contained",
                    h1 { "Unable to find the skill." }
                }
            }

        }

    }
}
