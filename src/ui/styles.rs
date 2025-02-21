use eframe::egui::{self, Button, Ui, Widget};

pub struct ButtonStyle;

impl ButtonStyle {
    pub fn primary(text: impl Into<String>) -> impl Widget {
        move |ui: &mut Ui| {
            Button::new(text.into())
                .fill(egui::Color32::from_rgb(25, 118, 210))
                .corner_radius(8.0)
                .min_size(egui::vec2(200.0, 50.0))
                .ui(ui)
        }
    }
}
