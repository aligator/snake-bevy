use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::app::AppState;
use crate::game;

use super::food;

mod components;
mod constants;
mod events;
mod resources;
mod systems;

#[derive(Debug, Clone, Eq, PartialEq, Hash, SystemSet)]
enum SnakeSet {
    Input,
    Movement,
    Action,
    Mutate,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::SnakeSegments::default())
            .insert_resource(resources::LastTailPosition::default())
            .add_event::<events::GrowthEvent>()
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
                    (
                        systems::snake_eating,
                        systems::snake_growth.after(systems::snake_eating),
                    )
                        .in_set(SnakeSet::Action)
                        .after(SnakeSet::Movement),
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
