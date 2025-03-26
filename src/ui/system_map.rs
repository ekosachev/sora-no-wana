use bevy::{
    color::Color,
    ecs::system::{Query, ResMut},
    log::{debug, warn},
};
use egui::{emath::RectTransform, vec2, Align2, Color32, Pos2, Rect, Sense, Stroke, Ui};

use crate::game::components::{
    planet::{BodyType, BodyTypes},
    system_map::{CelestialBodyData, SystemMap},
};

pub fn render_system_map(
    ui: &mut Ui,
    mut map: ResMut<SystemMap>,
    bodies: Query<&CelestialBodyData>,
) {
    let (response, painter) = ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

    let furthest_orbit = bodies
        .iter()
        .map(|b| (b.position.x.powi(2) + b.position.y.powi(2)).sqrt())
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or(10.0)
        * 1.62;
    let to_screen = RectTransform::from_to(
        Rect::from_x_y_ranges(
            (-furthest_orbit / map.zoom)..=(furthest_orbit / map.zoom),
            (-furthest_orbit / map.zoom)..=(furthest_orbit / map.zoom),
        )
        .scale_from_center2(vec2(response.rect.aspect_ratio(), 1.0)),
        response.rect,
    );

    painter.rect_filled(response.rect, 0.0, Color32::from_rgb(12, 12, 36));

    bodies.iter().for_each(|b| {
        let pos = to_screen.transform_pos(Pos2::new(b.position.x, b.position.y)) + map.position;
        let orbit_center =
            to_screen.transform_pos(Pos2::new(b.orbit_center.x, b.orbit_center.y)) + map.position;

        let color = match b.body_type {
            BodyTypes::Ring => Color32::GRAY,
            BodyTypes::AsteroidBelt => Color32::LIGHT_GRAY,
            BodyTypes::Planet => Color32::GREEN,
            BodyTypes::Moon => Color32::PURPLE,
            BodyTypes::GasGiant => Color32::RED,
        };

        painter.circle_stroke(
            orbit_center,
            b.orbit_radius * to_screen.scale().x,
            Stroke::new(1.0, Color32::WHITE),
        );

        painter.circle_filled(pos, 5.0, color);
    });
    if response.dragged() {
        map.position += response.drag_delta();
    }
    let scroll = ui.input(|i| i.smooth_scroll_delta);
    map.zoom *= 1.001f32.powf(scroll.y);
    map.zoom = map.zoom.clamp(0.01, 100.0);
}
