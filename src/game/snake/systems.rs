use bevy::prelude::*;

use super::*;
use crate::game;

pub fn spawn_snake(mut commands: Commands) {
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
        .insert(components::SnakeHead)
        .insert(game::components::Position { x: 3, y: 3 })
        .insert(game::components::Size::square(0.8));
}

pub fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut game::components::Position, With<components::SnakeHead>>,
) {
    for mut position in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            position.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            position.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            position.y += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            position.y -= 1;
        }
    }
}
