use eframe::egui::widgets::Button;
use eframe::egui::{self, Align, Layout, Rect, Vec2};

pub struct MainMenuScreen;

impl MainMenuScreen {
    pub fn show(ctx: &egui::Context) -> Option<Self> {
        let mut result = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.set_max_size(Vec2::new(100.0, 300.0));
                if ui.add(Button::new("Start example game")).clicked() {
                    result = Some(Self)
                };
            });
        });

        result
    }
}
