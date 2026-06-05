use dioxus::prelude::*;
use dioxus_icons::lucide::{CircleUserRound, ListCollapse, Moon, Sun};
use dioxus_motion::prelude::*;

use crate::{FAVICON, SVG_ICON_SIZE, Theme, components::*, nav::*};

#[derive(Routable, Clone, Debug, PartialEq, MotionTransitions)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
        #[route("/")]
        #[transition(Fade)]
        Home {},

        #[route("/profile/:id")]
        #[transition(Fade)]
        ProfileView { id: u32 },
        
        #[nest("/skill_lab")]

            #[route("")]
            #[transition(Fade)]
            BrowseSkill {},

            #[route("/create_skill")]
            #[transition(Fade)]
            CreateSkill {},
            
            #[route("/:id")]
            #[transition(Fade)]
            SkillView { id: u32 },

        #[end_nest]

        #[route("/about")]
        #[transition(Fade)]
        About {},

        #[route("/dev")]
        #[transition(Fade)]
        Dev {},

        #[nest("/skill_lab")]
            #[redirect("/", || Route::BrowseSkill {})]
            #[redirect("/create_skill", || Route::CreateSkill {})]
            #[redirect("/:id", |id: u32| Route::SkillView { id })]
        #[end_nest]

        #[route("/:..route")]
        PageNotFound { route: Vec<String> },
}

const COLLAPE_WHEN: u16 = 800;

/// Shared navbar component.
#[component]
fn NavBar() -> Element {
    let mut nav_width = use_signal(|| 500u16);
    let mut side_bar_activate = use_signal(|| false);
    let mut theme = use_context::<Signal<Theme>>();

    let toggle_theme = move |_| {
        let value = match theme() {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        };
        theme.set(value);
    };

    rsx! {

        if side_bar_activate() && nav_width() <= COLLAPE_WHEN {
            div { id: "nav_side_bar", class: "navbar", Nav_Component {} }
        }

        div {
            id: "navbar",
            class: "navbar",
            onresize: move |event| {
                let Ok(recty) = event.data.get_content_box_size() else {
                    return;
                };
                nav_width.set(recty.width as u16);
            },

            Left_Panel { nav_width }

            img { src: FAVICON, max_height: "90%", margin: "0 3rem 0 3rem" }
            div { flex_grow: 1, max_height: "100%" }

            if nav_width() > COLLAPE_WHEN {
                Nav_Component {}
            }

            div {
                class: "div_button",
                cursor: "pointer",
                onclick: toggle_theme,
                if matches!(theme(), Theme::Dark) {
                    Moon { size: SVG_ICON_SIZE }
                } else {
                    Sun { size: SVG_ICON_SIZE }
                }
            }

            Login {}

        }

        AnimatedOutlet::<Route> {}
    }
}

#[component]
fn Nav_Component() -> Element {
    rsx! {
        Link { to: Route::Home {}, "Home" }
        Link { to: Route::BrowseSkill {}, "Skill" }
        Link { to: Route::CreateSkill {}, "Create Skill" }
        // if cfg!(debug_assertions) {
        //     Link { to: Route::About {}, "About" }
        //     Link { to: Route::Dev {}, "Dev" }
        // }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

/// There must be better way to load css style, haven't found
///
/// This is placeholder to preload the style for button relate
#[component]
pub fn First_Loader() -> Element {
    rsx! {
        div { class: "hidden",
            Dev {}
            BrowseSkill {}
        }
    }
}

#[component]
fn Left_Panel(nav_width: Signal<u16>) -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        if nav_width() <= COLLAPE_WHEN {
            div { class: "div_button", onclick: move |_e| open.set(true),
                ListCollapse { size: SVG_ICON_SIZE }
            }
        }

        Sheet {
            open: open(),
            on_open_change: move |v| open.set(v),
            "data-side": SheetSide::Left.as_str(),
            SheetHeader {
                SheetTitle { "Skill Lab" }
                // SheetDescription { "Hi, this is Side panel." }
            }

            div {
                display: "grid",
                flex: "1 1 0%",
                grid_auto_rows: "min-content",
                gap: "1.5rem",
                padding: "0 1rem",

                Nav_Component {}
            }

            SheetContentClose {}
        }
    }
}
