use bevy::prelude::{Bundle, Component};

use super::{
    common::{Mass, Radius, ID},
    orbit::OrbitBundle,
};

#[derive(Bundle, Clone)]
pub struct CelestialBodyBundle {
    pub id: ID,
    pub system_id: SystemId,
    pub mass: Mass,
    pub radius: Radius,
    pub body_type: BodyType,
    pub orbit_bundle: OrbitBundle,
}

impl CelestialBodyBundle {
    pub fn density(&self) -> f32 {
        self.mass.0 / self.radius.0.powi(3) / std::f32::consts::PI * 4.0 / 3.0
    }
}

#[derive(Component, Clone)]
pub struct SystemId(pub ID);

#[derive(Component, Clone, Copy)]
pub struct BodyType(pub BodyTypes);

#[derive(Clone, Copy)]
pub enum BodyTypes {
    Planet,
    GasGiant,
    Moon,
    AsteroidBelt,
    Ring,
}
