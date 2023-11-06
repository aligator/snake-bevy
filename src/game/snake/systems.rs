use bevy::{prelude::*, window::PrimaryWindow};

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

pub fn size_scaling(
    mut query: Query<(&game::components::Size, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (sprite_size, mut transform) in query.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / game::constants::ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / game::constants::ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

pub fn position_translation(
    mut query: Query<(&game::components::Position, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = window_query.get_single().unwrap();

    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            convert(
                position.x as f32,
                window.width() as f32,
                game::constants::ARENA_WIDTH as f32,
            ),
            convert(
                position.y as f32,
                window.height() as f32,
                game::constants::ARENA_HEIGHT as f32,
            ),
            0.0,
        );
    }
}
