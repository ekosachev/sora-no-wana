use crate::game::components::system_map::{CelestialBodyData, SystemMap};
use crate::game::resourses::galaxy::GalaxyConfig;
use crate::game::resourses::system::SelectedSystem;
use crate::game::systems::ui::draw_galaxy_map;
use crate::ui::galaxy_map::Interaction;
use crate::ui::system_map::render_system_map;
use crate::ui::{CurrentScreen, GameScreenFlags, GameWindowTabs, Screen};
use crate::{game::components::galaxy_map::GalaxyMap, ui::Message};
use bevy::ecs::system::Query;
use bevy::prelude::{Res, ResMut};
use egui::{Align, Layout};

pub fn draw_game_screen(
    ctx: &egui::Context,
    mut map: ResMut<GalaxyMap>,
    mut system_map: ResMut<SystemMap>,
    system_bodies: Query<&CelestialBodyData>,
    mut selected_system: ResMut<SelectedSystem>,
    mut flags: ResMut<GameScreenFlags>,
    config: Res<GalaxyConfig>,
) -> Message {
    let mut result = Message::None;
    egui::TopBottomPanel::top("tab_menu")
        .resizable(false)
        .show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                if ui.button("Galaxy Map").clicked() {
                    flags.current_tab = GameWindowTabs::GalaxyMap;
                };

                if ui.button("System Map").clicked() {
                    flags.current_tab = GameWindowTabs::SystemMap;
                };
            });
        });
    match flags.current_tab {
        GameWindowTabs::GalaxyMap => {}
        GameWindowTabs::SystemMap => {
            let _ = egui::SidePanel::left("ssss").show(ctx, |ui| {});
        }
        _ => {}
    }
    let mut interaction = Interaction::None;
    match flags.current_tab {
        GameWindowTabs::GalaxyMap => {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    interaction = draw_galaxy_map(
                        ui,
                        map,
                        config,
                        selected_system,
                        system_map,
                        system_bodies,
                    );
                });
            });
        }
        GameWindowTabs::SystemMap => {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    render_system_map(ui, system_map, system_bodies);
                })
            });
        }
        _ => {}
    }

    if interaction == Interaction::GoToSystemMap {
        flags.current_tab = GameWindowTabs::SystemMap;
    }

    return result;
}
