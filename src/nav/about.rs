use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    let a = [
        ("https://github.com/Chssam", "Created by Chssam"),
        (
            "https://rust-lang.org/",
            "Programmed in modern language, Rust",
        ),
        ("https://dioxuslabs.com/", "Framework by Dioxus"),
        ("https://bevy.org/", "Sponsor to Bevy Game Engine"),
    ];

    rsx! {
        div {
            class: "center",
            flex_direction: "Column",
            margin_bottom: "1rem",
            div {

                h1 { "About" }

                p { "Wtf am I doing, spend straight many days just like this" }
                p { "This site is more overlook on function than design" }
                p { "Created and hosted for the assignment thingy." }
                p { "Dioxus the Framework used." }

                p { "Should have used 'App' instead of 'World'," }
                p { "This would make update simpler, it was my mistake," }
                p { "Skill is skill issues." }
            }
        }

        div { class: "center",
            div { id: "links",
                for (linked, content) in a {
                    a { href: "{linked}", target: "_blank", "{content}" }
                }
            }
        }

        div { class: "center",
            div { id: "gif",
                img {
                    class: "active",
                    max_width: "100%",
                    src: "https://tenor.com/view/guaton-computadora-enojado-computer-rage-gif-14480338.gif",
                }
            }
        }

        p { class: "center", "Spend few days to find solution that works to host" }
    }
}
