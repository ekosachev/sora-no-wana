use bevy::prelude::{Bundle, Component};
use vecmath::Vector2;

#[derive(Bundle, Default)]
pub struct SpatialBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

#[derive(Component, Default)]
pub struct Position(pub Vector2<f32>);

#[derive(Component, Default)]
pub struct Velocity(pub Vector2<f32>);

#[derive(Component, Default)]
pub struct Acceleration(pub Vector2<f32>);

impl SpatialBundle {
    pub fn from_position(pos: Vector2<f32>) -> Self {
        Self {
            position: Position(pos),
            ..Default::default()
        }
    }
}
