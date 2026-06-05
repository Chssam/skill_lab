use dioxus::prelude::*;

use crate::data::TheWorld;

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "center stat",
            Total_Stat {}
            Average_Stat {}
        }
    }
}

#[component]
fn Total_Stat() -> Element {
    let sig_world = use_context::<Signal<TheWorld>>();

    let total_stats = use_memo(move || {
        let world = sig_world.read();
        let total_user = world.total_user();
        let total_online_user = world.total_online_user();
        let total_time = world.total_time_all();
        let total_skill = world.total_skill();
        let total_review = world.total_review();

        let total_time_dt = time::Duration::saturating_seconds_f32(total_time);

        let texted = [
            format!("User: {}", total_user),
            format!("Online: {}", total_online_user),
            format!(
                "Time: {}D {}h {}m {}s",
                total_time_dt.whole_days(),
                total_time_dt.whole_hours() % 24,
                total_time_dt.whole_minutes() % 60,
                total_time_dt.whole_seconds() % 60
            ),
            format!("Skill: {}", total_skill),
            format!("Review: {}", total_review),
        ];

        texted
    });

    rsx! {
        div {
            h1 { "Total stats" }

            for text in total_stats() {
                p { "{text}" }
            }
        }
    }
}

#[component]
fn Average_Stat() -> Element {
    let sig_world = use_context::<Signal<TheWorld>>();

    let avg_stats = use_memo(move || {
        let world = sig_world.read();
        let avg_time = world.avg_time_all();
        let avg_skill = world.avg_skill();
        let avg_review = world.avg_review();

        let total_time_dt = time::Duration::saturating_seconds_f32(avg_time);

        let texted = [
            format!(
                "Time: {}D {}h {}m {}s",
                total_time_dt.whole_days(),
                total_time_dt.whole_hours() % 24,
                total_time_dt.whole_minutes() % 60,
                total_time_dt.whole_seconds() % 60
            ),
            format!("Skill: {:.2}", avg_skill),
            format!("Review: {:.2}", avg_review),
        ];

        texted
    });

    rsx! {
        div {
            h1 { "Average stats" }

            for text in avg_stats() {
                p { "{text}" }
            }
        }
    }
}
