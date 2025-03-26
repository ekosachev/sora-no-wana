use crate::game::components::galaxy_map::GalaxyMap;
use crate::ui::{ui_system, CurrentScreen, GameScreenFlags};
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use components::system_map::SystemMap;
use resourses::galaxy::GalaxyConfig;
use resourses::system::{SelectedSystem, SystemGenConfig};
use systems::galaxy_generation::generate_star_systems;
use systems::galaxy_map::update_galaxy_map;
use systems::star_system_generation::generate_plantary_systems;
use systems::system_map::update_system_map;

pub mod components;

pub mod resourses;
pub mod systems;

pub fn run_game(mut app: App) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Sora no Wana".into(),
            name: Some("sora_no_wana.app".into()),
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_plugins(EguiPlugin);

    app.insert_resource(GalaxyMap::default());
    app.insert_resource(SystemMap::default());
    app.insert_resource(SelectedSystem(None));
    app.insert_resource(GalaxyConfig::default());
    app.insert_resource(CurrentScreen::default());
    app.insert_resource(GameScreenFlags::default());
    app.insert_resource(SystemGenConfig::default());

    app.add_systems(
        Startup,
        (
            generate_star_systems,
            generate_plantary_systems,
            update_galaxy_map,
        )
            .chain(),
    );
    app.add_systems(Update, ui_system);
    app.add_systems(Update, update_system_map);

    app.run();
}
