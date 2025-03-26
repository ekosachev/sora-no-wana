use bevy::prelude::Resource;

use crate::game::components::common::ID;

#[derive(Resource)]
pub struct SystemGenConfig {
    pub min_bodies: u32,
    pub max_bodies: u32,
    pub log_mean_mass: f64,
    pub log_std_mass: f64,
    pub roche_limit_factor: f64,
}

impl Default for SystemGenConfig {
    fn default() -> Self {
        Self {
            min_bodies: 5,
            max_bodies: 20,
            log_mean_mass: 0.1,
            log_std_mass: 0.5,
            roche_limit_factor: 1.2,
        }
    }
}

#[derive(Resource)]
pub struct SelectedSystem(pub Option<ID>);
