use bevy::prelude::*;

use self::{food::FoodPlugin, snake::SnakePlugin};

mod components;
mod constants;
mod food;
mod snake;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SnakePlugin, FoodPlugin)).add_systems(
            PostUpdate,
            (systems::position_translation, systems::size_scaling),
        );
    }
}
