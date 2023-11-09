use bevy::prelude::*;

use super::*;
use crate::game;

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
    mut head_query: Query<(&mut game::components::Position, &components::SnakeHead)>,
) {
    let (mut position, head) = head_query.single_mut();
    match &head.direction {
        components::Direction::Left => {
            position.x -= 1;
        }
        components::Direction::Down => {
            position.y -= 1;
        }
        components::Direction::Right => {
            position.x += 1;
        }
        components::Direction::Up => {
            position.y += 1;
        }
    }
}
