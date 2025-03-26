use crate::{
    game::{
        components::{
            galaxy_map::GalaxyMap,
            system_map::{CelestialBodyData, SystemMap},
        },
        resourses::{galaxy::GalaxyConfig, system::SelectedSystem},
    },
    ui::galaxy_map::{render_galaxy_map, Interaction},
};
use bevy::{
    ecs::system::Query,
    prelude::{Res, ResMut},
};
use egui::{Frame, Ui};

pub fn draw_galaxy_map(
    mut ui: &mut Ui,
    map: ResMut<GalaxyMap>,
    config: Res<GalaxyConfig>,
    mut selected_system: ResMut<SelectedSystem>,
    mut system_map: ResMut<SystemMap>,
    celestial_bodies: Query<&CelestialBodyData>,
) -> Interaction {
    let mut interaction = Interaction::None;
    ui.centered_and_justified(|ui| {
        Frame::canvas(ui.style()).show(ui, |ui| {
            interaction = render_galaxy_map(
                ui,
                map,
                system_map,
                celestial_bodies,
                config,
                selected_system,
            );
        });
    });

    interaction
}
