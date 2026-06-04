use bevy_ecs::prelude::*;
use dioxus::prelude::*;

use crate::{
    data::{SkillCreated, SkillID, SkillMark, TheWorld, User, UserID},
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
            .try_query_filtered::<(&Name, &UserID, &SkillCreated), With<User>>()
            .map(|mut q| {
                q.iter(&world).find_map(|(name, q_id, op_skill_create)| {
                    q_id.0
                        .eq(&id)
                        .then(|| (name.to_string(), q_id.0, op_skill_create.clone()))
                })
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

            (v_1.0, v_1.1, ye)
        });

        out
    });

    if !the_world.read().has_current_user() {
        return rsx! {
            h1 { "Require login" }
        };
    }

    rsx! {
        if let Some((user_name, user_id, skill_created)) = me_name() {
            p { "Name: {user_name}" }
            p { "User ID: {user_id}" }
            p { "Skill created: {skill_created:?}" }
        } else {
            h1 { "Unable to find the user." }
        }
    }
}
