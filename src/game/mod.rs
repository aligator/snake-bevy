use bevy::prelude::*;

use self::snake::SnakePlugin;

mod components;
mod constants;
mod snake;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SnakePlugin);
    }
}
