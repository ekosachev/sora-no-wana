use crate::game::components::planet::CelestialBodyBundle;
use crate::game::components::system_map::CelestialBodyData;
use crate::game::components::{galaxy_map::GalaxyMap, system_map::SystemMap};
use crate::game::resourses::galaxy::GalaxyConfig;
use crate::game::resourses::system::SelectedSystem;
use crate::ui::screens::game_screen::draw_game_screen;
use crate::ui::screens::main_menu::draw_main_menu;
use bevy::ecs::system::Query;
use bevy::prelude::{default, Res, ResMut, Resource};
use bevy_egui::EguiContexts;

pub mod galaxy_map;
pub mod screens;
pub mod system_map;

#[derive(Default, Clone, Copy)]
pub enum Screen {
    #[default]
    MainMenu,
    Game,
}

#[derive(Resource, Default, Clone, Copy)]
pub struct CurrentScreen(pub Screen);

#[derive(Default, Resource)]
pub struct GameScreenFlags {
    pub current_tab: GameWindowTabs,
}

#[derive(Default)]
pub enum GameWindowTabs {
    #[default]
    GalaxyMap,
    SystemMap,
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut screen: ResMut<CurrentScreen>,
    mut map: ResMut<GalaxyMap>,
    mut system_map: ResMut<SystemMap>,
    system_bodies: Query<&CelestialBodyData>,
    mut game_screen_flags: ResMut<GameScreenFlags>,
    mut selected_system: ResMut<SelectedSystem>,
    config: Res<GalaxyConfig>,
) {
    let msg: Message = match screen.0 {
        Screen::MainMenu => draw_main_menu(contexts.ctx_mut()),
        Screen::Game => draw_game_screen(
            contexts.ctx_mut(),
            map,
            system_map,
            system_bodies,
            selected_system,
            game_screen_flags,
            config,
        ),
    };

    match msg {
        Message::StartGame => screen.0 = Screen::Game,
        _ => {}
    };
    // egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
    //     ui.label("World");
    // });
}

pub enum Message {
    StartGame,

    None,
}
