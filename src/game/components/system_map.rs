use bevy::ecs::{component::Component, system::Resource};
use egui::Vec2;

use super::{common::ID, planet::BodyTypes};

#[derive(Resource)]
pub struct SystemMap {
    pub system_id: Option<ID>,
    pub position: Vec2,
    pub zoom: f32,
}

impl Default for SystemMap {
    fn default() -> Self {
        Self {
            system_id: None,
            position: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

#[derive(Component, Clone)]
pub struct CelestialBodyData {
    pub position: Vec2,
    pub orbit_center: Vec2,
    pub orbit_radius: f32,
    pub body_type: BodyTypes,
    pub radius: f32,
}
