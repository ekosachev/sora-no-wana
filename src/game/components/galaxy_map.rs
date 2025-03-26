use bevy::prelude::Resource;
use egui::Vec2;
use std::sync::{Arc, Mutex};
use vecmath::Vector2;

use super::{
    common::ID,
    star::{StarClass, StarType},
};

#[derive(Resource)]
pub struct GalaxyMap {
    pub stars: Arc<Mutex<Vec<StarData>>>,
    pub position: Vec2,
    pub zoom: f32,
    pub hovered: Option<usize>,
}

impl Default for GalaxyMap {
    fn default() -> Self {
        Self {
            stars: Arc::<Mutex<Vec<StarData>>>::default(),
            position: Vec2::default(),
            zoom: 1.0,
            hovered: None,
        }
    }
}

#[derive(Clone)]
pub struct StarData {
    pub id: ID,
    pub position: Vector2<f32>,
    pub star_type: StarType,
    pub star_class: StarClass,
    // pub discovered: bool,
}
