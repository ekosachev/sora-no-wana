use bevy::prelude::{Bundle, Component, Entity};

use super::common::ID;

#[derive(Bundle, Clone)]
pub struct OrbitBundle {
    pub orbit_radius: OrbitRadius,
    pub orbit_period: OrbitPeriod,
    pub orbit_position: OrbitPosition,

    pub parent: Parent,
}

#[derive(Component, Clone)]
pub struct OrbitRadius(pub f64);

#[derive(Component, Clone)]
pub struct Parent(pub Option<ID>);

#[derive(Component, Clone)]
pub struct OrbitPeriod(pub f64);

#[derive(Component, Clone)]
pub struct OrbitPosition(pub f64);
