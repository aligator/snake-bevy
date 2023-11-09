use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::app::AppState;
use crate::game;

mod components;
mod constants;
mod resources;
mod systems;

#[derive(Debug, Clone, Eq, PartialEq, Hash, SystemSet)]
enum SnakeSet {
    Input,
    Movement,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::SnakeSegments::default())
            .add_systems(OnEnter(AppState::Game), systems::spawn_snake)
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

pub fn spawn_segment(mut commands: Commands, position: game::components::Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: constants::SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(components::SnakeSegment)
        .insert(position)
        .insert(game::components::Size::square(0.65))
        .id()
}
