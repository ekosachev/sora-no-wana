use egui::{Button, Vec2};

use crate::ui::Message;

pub fn draw_main_menu(ctx: &egui::Context) -> Message {
    let mut result = Message::None;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            ui.set_max_size(Vec2::new(300.0, 50.0));
            if ui.add(Button::new("Start example game")).clicked() {
                result = Message::StartGame;
            };
        });
    });
    return result;
}
