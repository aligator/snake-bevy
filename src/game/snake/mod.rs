use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::app::AppState;

mod components;
mod constants;
mod systems;

#[derive(Debug, Clone, Eq, PartialEq, Hash, SystemSet)]
enum SnakeSet {
    Input,
    Movement,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), systems::spawn_snake)
            .add_systems(
                Update,
                (
                    systems::snake_movement_input
                        .in_set(SnakeSet::Input)
                        .before(SnakeSet::Movement),
                    systems::snake_movement
                        .in_set(SnakeSet::Movement)
                        .run_if(on_timer(Duration::from_millis(150))),
                )
                    .run_if(in_state(AppState::Game)),
            );
        //.add_systems(OnExit(AppState::Game), despawn_snake);
    }
}
