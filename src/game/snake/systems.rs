use bevy::prelude::*;

use super::{resources::SnakeSegments, *};
use crate::game::{self, components::Position};

pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<resources::SnakeSegments>) {
    *segments = resources::SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: constants::SNAKE_HEAD_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            })
            .insert(components::SnakeHead {
                direction: components::Direction::Up,
            })
            .insert(game::components::Position { x: 3, y: 3 })
            .insert(game::components::Size::square(0.8))
            .id(),
        spawn_segment(commands, game::components::Position { x: 3, y: 2 }),
    ]);
}

pub fn snake_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_query: Query<&mut components::SnakeHead>,
) {
    let mut head = head_query.single_mut();
    let dir: components::Direction = if keyboard_input.pressed(KeyCode::Left) {
        components::Direction::Left
    } else if keyboard_input.pressed(KeyCode::Right) {
        components::Direction::Right
    } else if keyboard_input.pressed(KeyCode::Up) {
        components::Direction::Up
    } else if keyboard_input.pressed(KeyCode::Down) {
        components::Direction::Down
    } else {
        head.direction
    };
    if dir != head.direction.opposite() {
        head.direction = dir;
    }
}

pub fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut head_query: Query<(Entity, &components::SnakeHead)>,
    mut positions_query: Query<&mut Position>,
) {
    let (head_entity, head) = head_query.single_mut();
    let segment_positions = segments
        .iter()
        .map(|e| *positions_query.get_mut(*e).unwrap())
        .collect::<Vec<Position>>();

    let mut head_position = positions_query.get_mut(head_entity).unwrap();

    match &head.direction {
        components::Direction::Left => {
            head_position.x -= 1;
        }
        components::Direction::Right => {
            head_position.x += 1;
        }
        components::Direction::Up => {
            head_position.y += 1;
        }
        components::Direction::Down => {
            head_position.y -= 1;
        }
    }

    // This effectively sets the position of each segment to the segment in front of it.
    segment_positions
        .iter()
        .zip(segments.iter().skip(1))
        .for_each(|(position, segment)| {
            *positions_query.get_mut(*segment).unwrap() = *position;
        })
}
