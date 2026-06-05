use bevy_ecs::prelude::*;
use dioxus::prelude::*;

use crate::{
    components::{
        Input, Label,
        textarea::{Textarea, TextareaVariant},
    },
    data::*,
    nav::Route,
};

#[component]
pub fn ProfileView(id: u32) -> Element {
    let the_world = use_context::<Signal<TheWorld>>();

    let me_name = use_memo(move || {
        let world = the_world.read();
        let id = id;

        let op_q_skill = world.try_query_filtered::<(&Name, &SkillID), With<SkillMark>>();

        let v = world
            .try_query_filtered::<(
                &Name,
                &UserID,
                &SkillCreated,
                &Description,
                &ReviewCreated,
                &UserPicture,
            ), With<User>>()
            .map(|mut q| {
                q.iter(&world).find_map(
                    |(name, q_id, skill_create, description, review_created, img)| {
                        q_id.0.eq(&id).then(|| {
                            (
                                name.to_string(),
                                q_id.0,
                                skill_create.clone(),
                                description.0.clone(),
                                review_created.clone(),
                                img.clone(),
                            )
                        })
                    },
                )
            })
            .flatten();

        let out = v.map(|v_1| {
            let ye = op_q_skill
                .map(|mut q_s| {
                    let ab = v_1
                        .2
                        .iter()
                        .map_while(|ent| {
                            q_s.get(&world, ent).ok().map(|a| (a.0.to_string(), a.1.0))
                        })
                        .collect::<Vec<_>>();
                    ab
                })
                .unwrap_or_default();

            (v_1.0, v_1.1, ye, v_1.3, v_1.4, v_1.5)
        });

        out
    });

    if !the_world.read().has_current_user() {
        return rsx! {
            h1 { "Require login" }
        };
    }

    rsx! {
        div { class: "center", flex_direction: "column",

            if let Some((user_name, user_id, skill_created, description, review_created, img)) = me_name() {
                div {
                    class: "contained",
                    display: "flex",
                    flex_grow: 1,
                    flex_shrink: 0,
                    max_width: "100vh",
                    flex_wrap: "wrap",
                    flex_direction: "row",

                    div { margin_right: "1rem",
                        p { "Name: {user_name} ({user_id})" }
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
                        p { "Skill created: {skill_created.len()}" }
                        p { "Reviewed to: {review_created.len()}" }
                    }

                    div { class: "contained",
                        h1 { "Skill created:" }
                        for (name, id) in skill_created {
                            div {
                                Link { to: Route::SkillView { id: id }, "{name}" }
                            }
                        }

                        h3 { "Skill reviewed:" }
                    }
                }
            } else {
                div { class: "contained",
                    h1 { "Unable to find the user." }
                }
            }

        }

    }
}

#[component]
pub fn EditProfile() -> Element {
    let mut the_world = use_context::<Signal<TheWorld>>();
    let mut description = use_signal(String::new);
    let mut img_link = use_signal(String::new);

    if !the_world.read().has_current_user() {
        return rsx! {
            h1 { "Require login" }
        };
    }

    use_effect(move || {
        let world = the_world.read();
        let Some((descrip, img)) = world
            .try_query_filtered::<(&Description, &UserPicture), With<CurrentUser>>()
            .map(|mut q| q.single(&world).ok())
            .flatten()
        else {
            return;
        };
        description.set(descrip.0.clone());
        img_link.set(img.0.clone());
    });

    let set_description = move |e: FormEvent| {
        let mut world = the_world.write();
        let mut descrip = world
            .query_filtered::<&mut Description, With<CurrentUser>>()
            .single_mut(&mut world)
            .unwrap();
        descrip.0 = e.value();
    };

    let set_img = move |e: FormEvent| {
        let mut world = the_world.write();
        let mut img = world
            .query_filtered::<&mut UserPicture, With<CurrentUser>>()
            .single_mut(&mut world)
            .unwrap();
        img.0 = e.value();
    };

    rsx! {
        div { class: "center", flex_direction: "column",

            h1 { "Edit profile (Update as edit)" }

            div {
                min_width: "300px",
                max_width: "700px",
                flex_grow: 1,
                margin_bottom: "1rem",
                Label { html_for: "profile_description", "Description" }
                Textarea {
                    id: "profile_description",
                    variant: TextareaVariant::Default,
                    height: "160px",
                    placeholder: "Enter your description",
                    value: description(),
                    oninput: set_description,
                }

                div { margin_top: "1rem",

                    Label { html_for: "skill_banner", "Profile picture (Image Link)" }
                    Input {
                        oninput: set_img,
                        placeholder: "Image link",
                        value: img_link,
                    }
                }
            }
        }
    }
}
