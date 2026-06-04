mod components;
mod data;
mod nav;
mod playful;

use std::rc::Rc;

use async_std::task::sleep;
use bevy_ecs::prelude::*;
use bevy_reflect::{Enum, prelude::*};
use bevy_scene::DynamicScene;
use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing as _;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

use crate::data::{CurrentUser, SessionToken, TheWorld, document, wrap_state, wrap_window};
use nav::*;
use playful::*;

pub const FAVICON: Asset = asset!("/assets/Skill Lab favicon.ico");
pub const LOGO: Asset = asset!("/assets/Skill Lab Logo.svg");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const THEME_CSS: Asset = asset!("/assets/components-theme.css");
pub const PIX_FONT: Asset = asset!("/assets/fonts/DepartureMono-1.500/DepartureMono-Regular.woff2");

pub const BUTTON_CSS: Asset = asset!("/src/components/button/style.css");
pub const TOAST_CSS: Asset = asset!("/src/components/toast/style.css");

pub const SVG_ICON_SIZE: &str = "2.2rem";

fn main() {
    LaunchBuilder::new().launch(App);
}

#[derive(Reflect, Default, Clone, Debug)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

#[component]
fn App() -> Element {
    let tween = use_store(|| Tween {
        duration: std::time::Duration::from_millis(500),
        easing: easer::functions::Cubic::ease_in_out,
    });
    use_context_provider(move || tween);

    use_context_provider(|| Signal::new(FilterState::default()));

    theme_ready();
    world_ready();
    cursor_ready();
    sessioned();

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: THEME_CSS }

        document::Link { rel: "stylesheet", href: BUTTON_CSS }
        document::Link { rel: "stylesheet", href: TOAST_CSS }

        document::Link { rel: "icon", href: FAVICON }

        style {
            "@font-face {{ font-family: 'departureMono'; src: url('{PIX_FONT}') format('woff2'); font-weight: normal; font-style: normal; font-display: swap; }}"
        }

        First_Loader {}
        Playful {}

        Router::<Route> {}
    }
}

fn theme_ready() {
    let theme = wrap_state::<Theme>();
    use_effect(move || {
        let doc = document().document_element().unwrap();
        doc.set_attribute("data-theme", &theme().variant_name().to_lowercase())
            .unwrap();
    });
}

fn world_ready() {
    let mut the_world = use_context_provider(|| Signal::new(TheWorld::new()));

    use_future(move || async move {
        loop {
            sleep(std::time::Duration::from_secs(1)).await;
            let mut world = the_world.write();
            world.update();
        }
    });

    use_future(move || async move {
        loop {
            sleep(std::time::Duration::from_secs(5)).await;

            let Some(got_storage) = wrap_window().local_storage().ok().flatten() else {
                warn!("Unable to access local storage");
                continue;
            };

            let world = the_world.read();
            let app_registry = world.resource::<AppTypeRegistry>().clone();
            let registry = app_registry.read();

            let scene = DynamicScene::from_world(&world);
            let serialized_scene = scene.serialize(&registry).unwrap();

            if let Err(err) = got_storage.set_item("scene", &serialized_scene) {
                error!(?err, "Fail to save to storage");
            };
        }
    });
}

fn cursor_ready() {
    let mut the_world = use_context::<Signal<TheWorld>>();
    let mut cursor_pos = use_context_provider(|| Signal::new(CursorPos::default()));

    /// Using .forget() leak memory
    ///
    /// We need to retain the function without droppping at the end of closure
    ///
    /// Solution: https://github.com/wasm-bindgen/wasm-bindgen/discussions/3007
    #[allow(unused)]
    struct HoldIt(Rc<Closure<dyn FnMut(web_sys::MouseEvent)>>);

    use_future(move || async move {
        let mut world = the_world.write();
        let closured =
            Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                let mut writing = cursor_pos.write();
                writing.x = e.x();
                writing.y = e.y();
            });
        wrap_window().set_onmousemove(Some(closured.as_ref().unchecked_ref()));

        let rced = Rc::new(closured);
        world.insert_non_send_resource(HoldIt(rced));
    });
}

fn sessioned() {
    let session_token = wrap_state::<SessionToken>();
    let mut the_world = use_context::<Signal<TheWorld>>();

    use_effect(move || {
        let token = session_token.read();
        let mut world = the_world.write();

        let Some(ent) = world
            .try_query::<(Entity, &SessionToken)>()
            .map(|mut q| q.iter(&world).find_map(|v| v.1.0.eq(&token.0).then(|| v.0)))
            .flatten()
        else {
            return;
        };

        world.entity_mut(ent).insert(CurrentUser);
    });
}
