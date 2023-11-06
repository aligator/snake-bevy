use app::AppPlugin;
use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

pub mod app;
pub mod game;
pub mod main_menu;

fn main() {
    App::new()
        .add_plugins((AppPlugin, MainMenuPlugin, GamePlugin))
        .run()
}
