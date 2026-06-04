use bevy_derive::*;
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

#[derive(Reflect, Component, Default, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct Description(pub String);
