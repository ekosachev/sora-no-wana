use crate::game::run_game;
use bevy::app::App;

mod game;
mod ui;

fn main() {
    let app = App::new();

    run_game(app);
}
