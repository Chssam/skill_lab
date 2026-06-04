use crate::data::wrap_window;
use bevy_derive::*;
use bevy_math::{IRect, IVec2, ivec2};
use dioxus::prelude::*;
use dioxus_motion::prelude::*;

const PEEK: Asset = asset!("/assets/peek/Peek.png");
const PEEK_UP: Asset = asset!("/assets/peek/Peek Up.png");
const PEEK_LEFT: Asset = asset!("/assets/peek/Peek Left.png");
const PEEK_RIGHT: Asset = asset!("/assets/peek/Peek Right.png");
const PEEK_TOP_LEFT: Asset = asset!("/assets/peek/Peek Top Left.png");
const PEEK_TOP_RIGHT: Asset = asset!("/assets/peek/Peek Top Right.png");

const PONK: Asset = asset!("/assets/Ponk.png");

#[derive(Default, Clone, Debug, Deref, DerefMut)]
pub struct CursorPos(UIPos);

pub type UIPos = IVec2;

const PEEK_POS: IVec2 = ivec2(40, 0);
const PEEK_SCALE: i32 = 6;

#[component]
pub fn Playful() -> Element {
    let cur_pos = use_context::<Signal<CursorPos>>();
    let mut scale = use_motion(6f32);
    let mut peek_rect = use_signal(|| IRect::from_corners(PEEK_POS, PEEK_POS));

    let hover = move |_| {
        scale.animate_to(
            9.5,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    let unhover = move |_| {
        scale.animate_to(
            6.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    rsx! {

        div {
            img {
                id: "playful",
                left: "{PEEK_POS.x}px",
                bottom: "{PEEK_POS.y}px",
                pointer_events: "none",
                src: {
                    let height = wrap_window().inner_height().unwrap().as_f64().unwrap() as i32;
                    let recty = peek_rect();
                    let mut pos = cur_pos();
                    // Calculate from bottom
                    pos.y = height - pos.y;

                    if pos.x <= recty.min.x && pos.y >= recty.max.y {
                        PEEK_TOP_LEFT
                    } else if pos.x >= recty.max.x && pos.y >= recty.max.y {
                        PEEK_TOP_RIGHT
                    } else if pos.x >= recty.min.x && pos.x <= recty.max.x && pos.y >= recty.max.y {
                        PEEK_UP
                    } else if pos.x <= recty.min.x {
                        PEEK_LEFT
                    } else if pos.x >= recty.max.x {
                        PEEK_RIGHT
                    } else {
                        PEEK
                    }
                },
                transform: "scale({PEEK_SCALE})",
                transform_origin: "left bottom",
                onresize: move |event| {
                    let Ok(size) = event.data.get_content_box_size() else {
                        return;
                    };

                    let mut starting = IRect::from_corners(PEEK_POS, PEEK_POS);
                    let width = size.width as i32 * PEEK_SCALE;
                    let height = size.height as i32 * PEEK_SCALE;
                    starting = starting
                        .union_point(ivec2(PEEK_POS.x + width as i32, PEEK_POS.y + height as i32));
                    peek_rect.set(starting);
                },

            }

            img {
                id: "playful",
                bottom: "50px",
                right: "70px",
                src: PONK,
                transform: {
                    let v = scale.get_value();
                    format!("scale(-{v}, {v})")
                },
                onmouseenter: hover,
                onmouseleave: unhover,
            }
        }

    }
}
