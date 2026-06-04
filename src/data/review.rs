use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{entity::MapEntities, prelude::*};
use bevy_reflect::prelude::*;

#[derive(Reflect, Component)]
#[reflect(Component)]
#[require(SkillReviewFrom)]
pub struct ReviewMark;

/// User who review
#[derive(Reflect, Component, Clone, MapEntities)]
#[relationship(relationship_target = ReviewCreated)]
#[reflect(Component)]
pub struct ReviewOf(#[entities] pub Entity);

#[derive(Reflect, Component, Default, Clone, Deref, MapEntities)]
#[relationship_target(relationship = ReviewOf)]
#[reflect(Component)]
pub struct ReviewCreated(#[entities] Vec<Entity>);

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct UnReview;

#[derive(Reflect, Default, Component, Clone, Deref, MapEntities)]
#[relationship_target(relationship = SkillReviewOf)]
#[reflect(Component)]
pub struct SkillReviewFrom(#[entities] Vec<Entity>);

/// [Skill] that got review
#[derive(Reflect, Component, Clone, MapEntities)]
#[relationship(relationship_target = SkillReviewFrom)]
#[reflect(Component)]
pub struct SkillReviewOf(#[entities] pub Entity);

#[derive(Reflect, Component, Clone, Deref, DerefMut)]
#[reflect(Component)]
pub struct Review(pub String);

#[derive(Reflect, Component, Clone, Deref, DerefMut)]
#[reflect(Component)]
pub struct Rating(pub u8);
