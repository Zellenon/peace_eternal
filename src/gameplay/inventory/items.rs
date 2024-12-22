use bevy::{
    prelude::{Changed, Commands, Component, Entity, IntoSystem, Query},
    reflect::Reflect,
};

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
pub struct Nickname(pub String);

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
pub struct FlavorText(pub String);
