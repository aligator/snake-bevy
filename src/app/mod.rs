use bevy::{
    prelude::*,
    window::{close_on_esc, PrimaryWindow},
};

pub struct AppPlugin;

#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        let add_systems = app
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (500.0, 500.0).into(),
                    title: "Snake!".to_string(),
                    ..default()
                }),
                ..default()
            }))
            .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
            .add_state::<AppState>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, close_on_esc);
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle { ..default() });
}
