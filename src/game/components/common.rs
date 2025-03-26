use bevy::prelude::{Component, Entity};
use uuid::Uuid;

#[derive(Component, Default)]
pub struct Name(pub String);

#[derive(Component, Default)]
pub struct Temperature(pub f32);

#[derive(Component, Default, Clone, Copy)]
pub struct Radius(pub f32);

#[derive(Component, Default, Clone, Copy)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct PendingGeneration;

#[derive(Component, Clone, PartialEq)]
pub struct ID(pub String);

impl Default for ID {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}
