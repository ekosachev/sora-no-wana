use std::thread::current;

use bevy::{
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    log::tracing_subscriber::filter,
};
use egui::Vec2;

use crate::game::{
    components::{
        common::{Radius, ID},
        orbit::{OrbitPeriod, OrbitPosition, OrbitRadius, Parent},
        planet::{BodyType, SystemId},
        system_map::CelestialBodyData,
    },
    resourses::system::SelectedSystem,
};

pub fn update_system_map(
    system_id: Res<SelectedSystem>,
    bodies: Query<(
        &SystemId,
        &ID,
        &Parent,
        &OrbitRadius,
        &OrbitPosition,
        &OrbitPeriod,
        &BodyType,
        &Radius,
    )>,
    system_map_objecs: Query<Entity, With<CelestialBodyData>>,
    mut commands: Commands,
) {
    system_map_objecs
        .iter()
        .for_each(|e| commands.entity(e).despawn());

    if system_id.0.is_none() {
        return;
    }

    let current_bodies: Vec<(_, _, _, _, _, _, _, _)> = bodies
        .iter()
        .filter(|(s, _, _, _, _, _, _, _)| &s.0 == system_id.0.as_ref().unwrap())
        .collect();

    let orphans = current_bodies
        .iter()
        .filter(|(_, _, p, _, _, _, _, _)| p.0.is_none());

    let mut to_spawn = Vec::<(ID, CelestialBodyData)>::with_capacity(current_bodies.len());

    let mut added_ids = Vec::<ID>::new();
    orphans.for_each(|(_, id, _, o_r, o_p, _, b_t, r)| {
        to_spawn.push((
            ID(id.0.clone()),
            CelestialBodyData {
                position: Vec2::new(o_p.0.cos() as f32, o_p.0.sin() as f32) * (o_r.0 as f32),
                orbit_center: Vec2::ZERO,
                orbit_radius: o_r.0 as f32,
                body_type: b_t.0,
                radius: r.0,
            },
        ));
        added_ids.push(ID(id.0.clone()));
    });

    while to_spawn.len() < current_bodies.len() {
        added_ids = to_spawn.iter().map(|(id, _)| id.clone()).collect();
        current_bodies
            .iter()
            .filter(|(_, _, p, _, _, _, _, _)| p.0.is_some())
            .filter(|(_, id, _, _, _, _, _, _)| added_ids.iter().all(|added_id| added_id != *id))
            .filter(|(_, _, p, _, _, _, _, _)| {
                added_ids
                    .iter()
                    .any(|host_id| p.0.clone().unwrap() == host_id.clone())
            })
            .for_each(|(_, id, p, o_r, o_p, _, b_t, r)| {
                let mut pos = Vec2::new(o_p.0.cos() as f32, o_p.0.sin() as f32) * (o_r.0 as f32);
                let probably_parent = to_spawn
                    .iter()
                    .find(|(parent_id, _)| p.0.clone().unwrap() == parent_id.clone());
                let mut orbit_center = Vec2::ZERO;

                if let Some(parent) = probably_parent {
                    pos += parent.1.position;
                    orbit_center = parent.1.position;
                }

                to_spawn.push((
                    ID(id.0.clone()),
                    CelestialBodyData {
                        position: pos,
                        orbit_center,
                        orbit_radius: o_r.0 as f32,
                        body_type: b_t.0,
                        radius: r.0,
                    },
                ));
            });
    }

    to_spawn.iter().cloned().for_each(|(_, d)| {
        commands.spawn(d);
    });
}
