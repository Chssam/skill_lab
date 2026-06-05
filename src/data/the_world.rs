use bevy_derive::*;
use bevy_ecs::{entity::EntityHashMap, prelude::*};
use bevy_scene::serde::SceneDeserializer;
use dioxus::logger::tracing::info;
use serde::de::DeserializeSeed as _;
use web_time::{Instant, SystemTime, UNIX_EPOCH};

use crate::data::*;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct TheWorld(World);

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct LastTick(Instant);

impl TheWorld {
    pub fn new() -> Self {
        let mut new_world = TheWorld(World::new());
        let app_registry = AppTypeRegistry::default();
        app_registry.write().register_derived_types();
        new_world.insert_resource(app_registry);

        let time = Instant::now();
        new_world.insert_resource(LastTick(time));

        new_world.add_observer(observe_new_user);
        new_world.add_observer(observe_new_skill);

        TheWorld::load_scene(&mut new_world);

        new_world.flush();

        new_world
    }

    fn load_scene(world: &mut World) {
        let app_registry = world.resource::<AppTypeRegistry>().clone();
        let registry = app_registry.read();
        let Some(got_storage) = wrap_window().local_storage().ok().flatten() else {
            return;
        };

        let Some(bytes) = got_storage.get_item("scene").ok().flatten() else {
            return;
        };

        let mut deserializer = ron::de::Deserializer::from_bytes(&bytes.as_bytes()).unwrap();
        let scene_deserializer = SceneDeserializer {
            type_registry: &registry,
        };

        let dyn_scene = scene_deserializer
            .deserialize(&mut deserializer)
            .map_err(|e| deserializer.span_error(e))
            .unwrap();

        dyn_scene
            .write_to_world(world, &mut EntityHashMap::new())
            .unwrap();
    }

    // Something Repeat
    pub fn has_current_user(&self) -> bool {
        self.try_query::<&CurrentUser>()
            .is_some_and(|mut q| q.single(self).is_ok())
    }

    //Users related stats------------------------------------------------------------------------------------------
    pub fn total_user(&self) -> usize {
        self.try_query::<&User>()
            .map(|mut q| q.iter(self).len())
            .unwrap_or_default()
    }

    pub fn total_online_user(&self) -> usize {
        self.try_query_filtered::<&User, With<Online>>()
            .map(|mut q| q.iter(self).len())
            .unwrap_or_default()
    }

    pub fn total_time_all(&self) -> f32 {
        self.try_query_filtered::<&TimeSpend, With<User>>()
            .map(|mut q| {
                let total_time_spend = q.iter(self).map(|t_s| t_s.as_secs_f32()).sum::<f32>();
                total_time_spend.abs()
            })
            .unwrap_or_default()
    }

    pub fn total_skill(&self) -> usize {
        self.try_query::<&SkillMark>()
            .map(|mut q| q.iter(self).len())
            .unwrap_or_default()
    }

    pub fn total_review(&self) -> usize {
        self.try_query_filtered::<&ReviewMark, Without<UnReview>>()
            .map(|mut q| q.iter(self).len())
            .unwrap_or_default()
    }

    pub fn avg_time_all(&self) -> f32 {
        self.try_query_filtered::<&TimeSpend, With<User>>()
            .map(|mut q| {
                let q_time_iter = q.iter(self);

                let total_user = q_time_iter.len();
                if total_user == 0 {
                    return 0.0;
                }

                let total_time_spend = q_time_iter.map(|t_s| t_s.as_secs_f32()).sum::<f32>();

                let avg_time = total_time_spend / total_user as f32;
                avg_time
            })
            .unwrap_or_default()
    }

    pub fn avg_skill(&self) -> f32 {
        let total_user = self.total_user();
        if total_user == 0 {
            return 0.0;
        }
        let total_skill = self.total_skill();

        total_skill as f32 / total_user as f32
    }

    pub fn avg_review(&self) -> f32 {
        let total_user = self.total_user();
        if total_user == 0 {
            return 0.0;
        }
        let total_review = self.total_review();

        total_review as f32 / total_user as f32
    }
    //Users related stats------------------------------------------------------------------------------------------

    pub fn update(&mut self) {
        let mut last_tick = self.resource_mut::<LastTick>();

        let elapsed = last_tick.elapsed();
        last_tick.0 = Instant::now();

        if let Some(mut q_time_spend) =
            self.try_query_filtered::<&mut TimeSpend, (With<User>, With<Online>)>()
        {
            let q_time_iter = q_time_spend.iter_mut(self);
            q_time_iter.for_each(|mut t_s| {
                t_s.0 = t_s.saturating_add(elapsed);
            });
        };

        if let Some(mut q_session) =
            self.try_query_filtered::<(Entity, &mut SessionDuration), With<User>>()
        {
            let mut expire_session: Vec<Entity> = Vec::with_capacity(2);
            let q_session_iter = q_session.iter_mut(self);
            q_session_iter.for_each(|(ent, mut t_s)| {
                if t_s.tick(elapsed).is_finished() {
                    expire_session.push(ent);
                }
            });
            expire_session.into_iter().for_each(|ent| {
                self.entity_mut(ent)
                    .remove_with_requires::<(SessionToken, CurrentUser)>();
            });
        };

        let mut del = Vec::with_capacity(1);
        self.query_filtered::<EntityRef, (Without<UserID>, Without<SkillID>)>()
            .iter(&self)
            .for_each(|e| {
                if e.archetype().component_count() == 0 {
                    del.push(e.entity());
                }
            });
        del.into_iter().for_each(|e| {
            self.entity_mut(e).despawn();
        });

        self.flush();
    }
}

fn observe_new_user(
    on_user: On<Add, User>,
    q_user: Query<(Has<UserID>, Has<RegisteredDate>), With<User>>,
    mut cmd: Commands,
) {
    let ent = on_user.entity;
    let Ok((has_id, has_register)) = q_user.get(ent) else {
        return;
    };

    let mut ent_mut = cmd.entity(ent);
    if !has_id {
        let new_user_id = q_user.iter().len() as u32;
        ent_mut.insert(UserID::new(new_user_id));
    }
    if !has_register {
        let today = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        ent_mut.insert(RegisteredDate(today));
    }
}

fn observe_new_skill(
    on_skill: On<Add, SkillMark>,
    q_skill: Query<Has<SkillID>, With<SkillMark>>,
    mut cmd: Commands,
) {
    let ent = on_skill.entity;
    let Ok(has_id) = q_skill.get(ent) else {
        return;
    };

    let mut ent_mut = cmd.entity(ent);
    if !has_id {
        let new_skill_id = q_skill.iter().len() as u32;
        ent_mut.insert(SkillID(new_skill_id));
    }
}
