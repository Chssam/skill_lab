mod review;
mod shared;
mod skills;
mod students;
mod the_world;

pub use review::*;
pub use shared::*;
pub use skills::*;
pub use students::*;
pub use the_world::*;
pub use via::*;

mod via {
    use bevy_reflect::{
        GetTypeRegistration, TypeRegistration, TypeRegistry, Typed,
        prelude::*,
        serde::{TypedReflectDeserializer, TypedReflectSerializer},
    };
    use dioxus::prelude::*;
    use serde::de::DeserializeSeed as _;
    use web_sys::{Document, Window, window};

    pub fn wrap_state<
        T: Reflect
            + Typed
            + TypePath
            + FromReflect
            + GetTypeRegistration
            + Default
            + Clone
            + std::fmt::Debug,
    >() -> Signal<T> {
        let signal_t = use_context_provider(|| {
            let value_t: T = match wrap_window().local_storage() {
                Ok(Some(got_storage)) => {
                    let a = got_storage.get(T::short_type_path()).ok().flatten();
                    a.map(|data| {
                        let mut registry = TypeRegistry::default();
                        registry.register::<T>();

                        let registration = TypeRegistration::of::<T>();
                        let reflect_deserializer =
                            TypedReflectDeserializer::new(&registration, &registry);

                        let mut value = ron::Deserializer::from_bytes(data.as_bytes()).unwrap();
                        let reflect_value = reflect_deserializer.deserialize(&mut value).unwrap();
                        let reflected_type = <T as FromReflect>::take_from_reflect(reflect_value);

                        reflected_type.unwrap()
                    })
                }
                Ok(None) => {
                    warn!("Local storage return None.");
                    None
                }
                Err(err) => {
                    warn!(?err, "No access to local storage");
                    None
                }
            }
            .unwrap_or_default();

            Signal::new(value_t)
        });

        use_effect(move || {
            let data = signal_t.read();

            let mut registry = TypeRegistry::default();
            registry.register::<T>();

            let serializer = TypedReflectSerializer::new(data.as_partial_reflect(), &registry);

            let well = match ron::ser::to_string(&serializer) {
                Ok(ready) => ready,
                Err(err) => return warn!("FAILED TO CONVERT: {:#?}", err),
            };

            // info!("Updated ({}): {}", T::short_type_path(), well);

            if let Some(got_storage) = wrap_window().local_storage().ok().flatten() {
                if let Err(err) = got_storage.set_item(T::short_type_path(), &well) {
                    error!(?err, "Fail to save to storage");
                };
            }
        });

        signal_t
    }

    pub fn wrap_window() -> Window {
        window().unwrap()
    }

    pub fn document() -> Document {
        wrap_window().document().unwrap()
    }
}
