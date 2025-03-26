use bevy::prelude::{Commands, Res};
use core::f32;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use vecmath::Vector2;

use crate::game::{
    components::{
        common::PendingGeneration,
        spatial::SpatialBundle,
        star::{Star, StarBundle},
    },
    resourses::galaxy::GalaxyConfig,
};

pub fn generate_system_positions(config: Res<GalaxyConfig>) -> Vec<Vector2<f32>> {
    let perlin = Perlin::new(config.seed);
    let mut rng = rand::rng();
    let mut positions = Vec::<Vector2<f32>>::with_capacity(config.num_stars);

    for _ in 0..config.num_stars {
        let base_angle = rng.random_range(0.0..2.0 * std::f32::consts::PI);

        let r = config.galaxy_radius * f32::consts::E.powf(base_angle * 0.5);
        let spiral_spread = config.arm_strength * (r + 1.0).ln();

        let spiral_offset = f32::consts::TAU / config.arm_count as f32
            * rng.random_range(0..=config.arm_count) as f32;

        let nx = config.noise_scale * r * base_angle.cos();
        let ny = config.noise_scale * r * base_angle.sin();
        let noise_val = perlin.get([nx as f64, ny as f64]) as f32;

        let r_final = r * (1.0 + 0.5 * noise_val) + spiral_spread;
        let angle_final = base_angle + 0.5 * noise_val + spiral_offset;

        positions.push([r_final * angle_final.cos(), r_final * angle_final.sin()]);
    }
    positions
}

pub fn generate_star_systems(mut commands: Commands, config: Res<GalaxyConfig>) {
    let positions = generate_system_positions(config);

    for position in positions {
        commands.spawn((Star::generate(), SpatialBundle::from_position(position)));
    }
}
