use std::collections::HashSet;

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{entity::MapEntities, prelude::*};
use bevy_reflect::prelude::*;
use strum::{AsRefStr, Display, EnumCount, EnumIter};

use crate::data::{Description, SkillReviewFrom};

#[derive(Reflect, Component, Clone)]
#[require(Description, SkillImage, SkillTag, SkillReviewFrom)]
#[reflect(Component)]
pub struct SkillMark;

#[derive(Reflect, Component, Default, Clone, Debug, PartialEq, Deref, MapEntities)]
#[relationship_target(relationship = SkillCreatedBy)]
#[reflect(Component)]
pub struct SkillCreated(#[entities] Vec<Entity>);

#[derive(Reflect, Component, Clone, Debug, Deref, MapEntities)]
#[relationship(relationship_target = SkillCreated)]
#[reflect(Component)]
pub struct SkillCreatedBy(#[entities] pub Entity);

#[derive(Reflect, Component, Default, Clone, Deref, DerefMut)]
#[reflect(Component)]
pub struct SkillImage(pub String);

#[derive(Reflect, Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deref)]
#[component(immutable)]
#[reflect(Component)]
pub struct SkillID(pub u32);

#[derive(Reflect, Component, Default, Clone, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct SkillTag(pub HashSet<Skill>);

#[derive(
    Reflect,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumCount,
    EnumIter,
    Display,
    AsRefStr,
)]
pub enum Skill {
    Artist,
    Programming,
    Designer,
    ContentCreator,
    Communication,
    Electronical,
}

impl Skill {
    /// TODO!: Add emoji
    pub const fn emoji(&self) -> &'static str {
        match self {
            Skill::Artist => "",
            Skill::Programming => "",
            Skill::Designer => "",
            Skill::ContentCreator => "",
            Skill::Communication => "",
            Skill::Electronical => "",
        }
    }
}

pub struct QueueToTakeSkill;
