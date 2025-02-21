use std::default;

use crate::app::screens::main_menu::MainMenuScreen;
use eframe::egui::{self, debug_text};

mod screens;

#[derive(Default)]
pub struct GameApp {
    current_screen: Screens,
}

#[derive(Default)]
enum Screens {
    #[default]
    MainMenu,
    Game,
}

impl From<MainMenuScreen> for Screens {
    fn from(_: MainMenuScreen) -> Self {
        Screens::MainMenu
    }
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.current_screen {
            Screens::MainMenu => {
                if let Some(screen) = MainMenuScreen::show(ctx) {
                    self.current_screen = screen.into();
                }
            }
            Screens::Game => { /* ... */ }
        }
    }
}
