use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::app::AppState;

mod components;
mod constants;
mod systems;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (systems::food_spawner)
                .run_if(in_state(AppState::Game))
                .run_if(on_timer(Duration::from_secs(1))),
        );
        //.add_systems(OnExit(AppState::Game), despawn_snake);
    }
}
