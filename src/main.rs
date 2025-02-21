use app::GameApp;
use eframe::{egui, NativeOptions};

mod app;
mod ui;

fn main() -> eframe::Result {
    let options = NativeOptions::default();

    eframe::run_native(
        "Sora no Wana",
        options,
        Box::new(|_cc| Ok(Box::new(GameApp::default()))),
    )
}

