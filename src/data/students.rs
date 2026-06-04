use std::time::Duration;

use bevy_derive::*;
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_time::{Timer, TimerMode};

use crate::data::{Description, ReviewCreated, SkillCreated};

#[derive(Bundle)]
pub struct RegisterUserBundle {
    pub user: User,
    pub name: Name,
    pub login_name: LoginName,
    pub password: Password,
}

#[derive(Component)]
#[require(Online)]
pub struct CurrentUser;

#[derive(Reflect, Component, Clone, PartialEq, Deref)]
#[component(immutable)]
#[reflect(Component)]
pub struct UserID(pub u32);

impl UserID {
    pub fn new(n: u32) -> Self {
        Self(n)
    }
}

#[derive(Reflect, Component)]
#[require(TimeSpend, Description, SkillCreated, ReviewCreated)]
#[reflect(Component)]
pub struct User;

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Student;

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Admin;

#[derive(Component, Default)]
pub struct Online;

#[derive(Reflect, Component, Default, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct TimeSpend(pub Duration);

#[derive(Reflect, Component, Clone, Deref)]
#[component(immutable)]
#[reflect(Component)]
pub struct RegisteredDate(pub Duration);

/// Shouldn't be raw String
///
/// but it's showcase so let it be
#[derive(Reflect, Component, Default, Debug, Deref)]
#[component(immutable)]
#[reflect(Component)]
pub struct Password(pub String);

#[derive(Reflect, Component, Default, Debug, Deref)]
#[component(immutable)]
#[reflect(Component)]
pub struct LoginName(pub String);

#[derive(Reflect, Component, Clone, Default, Debug, Deref)]
#[component(immutable)]
#[require(SessionDuration)]
#[reflect(Component)]
pub struct SessionToken(pub String);

#[derive(Reflect, Component, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct SessionDuration(pub Timer);

impl Default for SessionDuration {
    fn default() -> Self {
        Self(Timer::new(Duration::from_mins(10), TimerMode::Once))
    }
}

// #[derive(Reflect, Component, Debug)]
// #[component(immutable)]
// #[reflect(Component)]
// pub enum Programme {
//     BIT,
//     BCS,
//     DIT,
//     DCS,
//     Other([char; 3]),
// }

#[derive(Reflect, Component, Debug, Deref)]
#[component(immutable)]
#[reflect(Component)]
pub struct Age(pub u8);

#[derive(Reflect, Component, Clone, Debug, PartialEq, Deref)]
#[component(immutable)]
#[reflect(Component)]
pub struct UserPicture(pub String);
