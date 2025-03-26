use crate::game::components::star::{StarClass, StarType};
use crate::game::components::system_map::{CelestialBodyData, SystemMap};
use crate::game::resourses::system::SelectedSystem;
use crate::game::{components::galaxy_map::GalaxyMap, resourses::galaxy::GalaxyConfig};
use bevy::ecs::system::Query;
use bevy::log::tracing_subscriber::fmt::format;
use bevy::prelude::{Res, ResMut};
use egui::emath::RectTransform;
use egui::{
    pos2, show_tooltip, vec2, Align2, Color32, FontId, Label, Pos2, Rect, Sense, Stroke,
    TextWrapMode, Ui, Vec2,
};

use super::system_map::render_system_map;

#[derive(PartialEq)]
pub enum Interaction {
    None,
    GoToSystemMap,
}

pub fn render_galaxy_map(
    ui: &mut Ui,
    mut map: ResMut<GalaxyMap>,
    mut system_map: ResMut<SystemMap>,
    celestial_bodies: Query<&CelestialBodyData>,
    config: Res<GalaxyConfig>,
    mut selected_system: ResMut<SelectedSystem>,
) -> Interaction {
    let mut result = Interaction::None;
    let (response, painter) =
        ui.allocate_painter(ui.available_size_before_wrap(), Sense::click_and_drag());
    let stars = map.stars.lock().unwrap();

    let to_screen = RectTransform::from_to(
        Rect::from_x_y_ranges(
            (-config.galaxy_radius / map.zoom)..=(config.galaxy_radius / map.zoom),
            (-config.galaxy_radius / map.zoom)..=(config.galaxy_radius / map.zoom),
        )
        .scale_from_center2(vec2(response.rect.aspect_ratio(), 1.0)),
        response.rect,
    );

    painter.rect_filled(response.rect, 0.0, Color32::from_rgb(12, 12, 36));

    for star in stars.iter() {
        let pos = to_screen.transform_pos(pos2(star.position[0], star.position[1])) + map.position;
        painter.circle_filled(
            pos,
            match star.star_class {
                StarClass::O => 7.0,
                StarClass::I => 6.0,
                StarClass::II => 5.0,
                StarClass::III => 4.0,
                StarClass::IV => 3.0,
                StarClass::V => 2.0,
            },
            (match star.star_type {
                StarType::O => Color32::from_hex("#92b5ff"),
                StarType::B => Color32::from_hex("#a2c0ff"),
                StarType::A => Color32::from_hex("#d5e0ff"),
                StarType::F => Color32::from_hex("#f9f5ff"),
                StarType::G => Color32::from_hex("#ffede3"),
                StarType::K => Color32::from_hex("#ffdab5"),
                StarType::M => Color32::from_hex("#ffb56c"),
            })
            .unwrap(),
        );
    }

    if let Some(star_index) = map.hovered {
        let star = stars[star_index].clone();
        selected_system.0 = Some(star.id.clone());
        let pos = to_screen.transform_pos(pos2(star.position[0], star.position[1])) + map.position;
        painter.circle_stroke(pos, 10.0, Stroke::new(4.0, Color32::RED));
        show_tooltip(ui.ctx(), ui.layer_id(), "system_info_popup".into(), |ui| {
            // ui.label(format!("\"Name\" star"));
            ui.add(Label::new("\"Name\" star"));
            // ui.label(format!("Spectral type: {}", star.star_type));
            ui.add(
                Label::new(format!("Spectral class: {}", star.star_type))
                    .wrap_mode(TextWrapMode::Extend),
            );
            // ui.label(format!("Class: {}", star.star_class));
            ui.add(
                Label::new(format!("Luminosity class: {}", star.star_class))
                    .wrap_mode(TextWrapMode::Extend),
            );
            ui.allocate_ui(vec2(400.0, 300.0), |ui| {
                render_system_map(ui, system_map, celestial_bodies)
            })
        });
    }
    let hovered_index: Option<usize> = if response.hovered() && response.hover_pos().is_some() {
        if let Some(star_in_radius) = stars.iter().enumerate().find(|(_, s)| {
            response
                .hover_pos()
                .unwrap()
                .distance(
                    to_screen.transform_pos(Pos2::new(s.position[0], s.position[1])) + map.position,
                )
                .abs()
                < 10.0
        }) {
            Some(star_in_radius.0)
        } else {
            None
        }
    } else {
        None
    };

    drop(stars);

    if response.dragged() {
        map.position += response.drag_delta();
    }

    if response.clicked() && map.hovered.is_some() {
        result = Interaction::GoToSystemMap;
    }

    let scroll = ui.input(|i| i.smooth_scroll_delta);
    map.zoom *= 1.01f32.powf(scroll.y);
    map.zoom = map.zoom.clamp(0.01, 1.0);
    map.hovered = hovered_index;
    result
}
