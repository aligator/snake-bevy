use bevy::prelude::*;
use crate::game::events::GameOverEvent;

use self::{food::FoodPlugin, snake::SnakePlugin};

mod components;
mod constants;
mod food;
mod snake;
mod systems;
mod events;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .add_plugins((SnakePlugin, FoodPlugin)).add_systems(
            PostUpdate,
            (
                systems::position_translation, 
                systems::size_scaling,
                systems::game_over
            ),
        );
    }
}
