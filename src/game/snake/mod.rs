use bevy::prelude::*;

use crate::app::AppState;

mod components;
mod constants;
mod systems;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), systems::spawn_snake)
            .add_systems(
                Update,
                (systems::snake_movement,).run_if(in_state(AppState::Game)),
            );
        //.add_systems(OnExit(AppState::Game), despawn_snake);
    }
}
