use bevy::prelude::Resource;

#[derive(Debug, Resource)]
pub struct GalaxyConfig {
    pub seed: u32,
    pub num_stars: usize,
    pub galaxy_radius: f32,
    pub arm_strength: f32,
    pub arm_count: u32,
    pub noise_scale: f32,
}

impl Default for GalaxyConfig {
    fn default() -> Self {
        Self {
            seed: 20250222,
            num_stars: 1000,
            galaxy_radius: 1500.0,
            arm_strength: 1.0,
            arm_count: 5,
            noise_scale: 1.0,
        }
    }
}
