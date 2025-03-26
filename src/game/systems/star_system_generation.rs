use core::f64;

use bevy::prelude::{BuildChildren, ChildBuild, Commands, Entity, Query, Res, With};
use itertools::Itertools;
use rand::Rng;
use rand_distr::Distribution;

use crate::game::{
    components::{
        common::{Mass, Radius, ID},
        orbit::{OrbitBundle, OrbitPeriod, OrbitPosition, OrbitRadius, Parent},
        planet::{BodyType, BodyTypes, CelestialBodyBundle, SystemId},
        star::Star,
    },
    resourses::system::SystemGenConfig,
};

pub fn generate_plantary_systems(
    mut commands: Commands,
    config: Res<SystemGenConfig>,
    star_query: Query<(Entity, &ID), With<Star>>,
) {
    let mut rng = rand::rng();

    star_query.iter().for_each(|(star_entity, id)| {
        let num_bodies = rng.random_range(config.min_bodies..=config.max_bodies);
        let mut bodies = (0..num_bodies)
            .map(|_| generate_celestial_body(&mut rng, &config, id.clone()))
            .collect::<Vec<_>>();

        bodies.sort_unstable_by(|a, b| b.mass.0.total_cmp(&a.mass.0));

        assign_moons(&mut bodies, &config);

        resolve_roche_limits(&mut bodies, &config);

        commands.entity(star_entity).with_children(|parent| {
            for body in bodies {
                parent.spawn(body);
            }
        });
    })
}

fn generate_celestial_body(
    rng: &mut impl Rng,
    config: &SystemGenConfig,
    star_id: ID,
) -> CelestialBodyBundle {
    let log_normal = rand_distr::LogNormal::new(config.log_mean_mass, config.log_std_mass).unwrap();
    let mass = log_normal.sample(rng) as f32 * 5.976e24;

    CelestialBodyBundle {
        id: ID::default(),
        system_id: SystemId(star_id),
        mass: Mass(mass),
        radius: Radius(mass.powf(0.3) * 6371.0),
        orbit_bundle: OrbitBundle {
            orbit_radius: OrbitRadius(rng.random_range(0.1..1500.0) * 149597870700.0),
            orbit_position: OrbitPosition(rng.random_range(0.0..f64::consts::TAU)),
            orbit_period: OrbitPeriod(0.0),
            parent: Parent(None),
        },
        body_type: BodyType(if mass > 0.1 * 1.898e27 {
            BodyTypes::GasGiant
        } else {
            BodyTypes::Planet
        }),
    }
}

static SOLAR_MASS: f64 = 1.989e30;

fn assign_moons(bodies: &mut [CelestialBodyBundle], config: &SystemGenConfig) {
    let mut rng = rand::rng();
    let mut moon_assignments = Vec::new();

    // Первый проход: собираем информацию о лунах
    for i in 0..bodies.len() {
        let parent = &bodies[i];

        let hill_radius = parent.orbit_bundle.orbit_radius.0
            * (parent.mass.0 as f64 / (3.0 * SOLAR_MASS)).powf(1.0 / 3.0);

        // Проверяем последующие тела
        for j in (i + 1)..bodies.len() {
            let child = &bodies[j];
            if child.orbit_bundle.parent.0.is_some() {
                continue;
            }

            let distance =
                (child.orbit_bundle.orbit_radius.0 - parent.orbit_bundle.orbit_radius.0).abs();
            if distance < hill_radius && child.mass.0 < parent.mass.0 {
                moon_assignments.push((i, j, distance));
            }
        }
    }

    // Второй проход: применяем изменения
    for (parent_idx, child_idx, distance) in moon_assignments {
        let parent_entity = bodies[parent_idx].id.clone();
        let parent_mass = bodies[parent_idx].mass;
        let parent_radius = bodies[parent_idx].radius;

        let child = &mut bodies[child_idx];
        child.orbit_bundle.parent.0 = Some(parent_entity);
        child.orbit_bundle.orbit_radius.0 =
            generate_moon_orbit(&mut rng, parent_mass.0, parent_radius.0, distance);
    }
}

// Обновленная функция генерации орбиты лун
fn generate_moon_orbit(
    rng: &mut impl Rng,
    parent_mass: f32,
    parent_radius: f32,
    max_distance: f64,
) -> f64 {
    let roche_limit =
        2.44 * parent_radius as f64 * (parent_mass as f64 / SOLAR_MASS).powf(1.0 / 3.0);

    if roche_limit >= max_distance {
        return 0.0;
    }
    rng.random_range(roche_limit..max_distance)
}

fn resolve_roche_limits(bodies: &mut Vec<CelestialBodyBundle>, config: &SystemGenConfig) {
    let mut to_remove = Vec::new();
    let mut rng = rand::rng();

    for (i, body) in bodies.iter().enumerate() {
        if let Some(parent_entity) = body.orbit_bundle.parent.0.clone() {
            if let Some(parent) = bodies.iter().find(|b| b.id == parent_entity) {
                let roche_limit =
                    2.44 * parent.radius.0 * (parent.density() / body.density()).powf(1.0 / 3.0);

                if (body.orbit_bundle.orbit_radius.0 as f32)
                    < roche_limit * config.roche_limit_factor as f32
                {
                    to_remove.push(i);
                }
            }
        }
    }

    // Удаляем или заменяем на кольца
    for &i in to_remove.iter().rev() {
        bodies[i].body_type.0 = BodyTypes::Ring;
    }
}
