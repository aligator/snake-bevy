use bevy::prelude::*;
use rand::random;

use crate::game;

use super::components;
use super::constants;

pub fn food_spawner(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: constants::FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(components::Food)
        .insert(game::components::Position {
            x: (random::<f32>() * game::constants::ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * game::constants::ARENA_HEIGHT as f32) as i32,
        })
        .insert(game::components::Size::square(0.8));
}

pub fn despawn_food(mut commands: Commands, food_query: Query<Entity, With<components::Food>>) {
    for food in food_query.iter() {
        commands.entity(food).despawn_recursive();
    }
}