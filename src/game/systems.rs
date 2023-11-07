use bevy::{prelude::*, window::PrimaryWindow};

use super::{components, constants};

pub fn size_scaling(
    mut query: Query<(&components::Size, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (sprite_size, mut transform) in query.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / constants::ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / constants::ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

pub fn position_translation(
    mut query: Query<(&components::Position, &mut Transform)>,
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
                constants::ARENA_WIDTH as f32,
            ),
            convert(
                position.y as f32,
                window.height() as f32,
                constants::ARENA_HEIGHT as f32,
            ),
            0.0,
        );
    }
}
