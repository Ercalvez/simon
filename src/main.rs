use bevy::{prelude::*, window::PresentMode};

mod game;
mod menu;
mod utils;
mod splash;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}


fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}



fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(
        WindowPlugin {
            primary_window: Some(Window {
                title: "Simon".to_string(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        })
    )
    .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
    .add_systems(Startup, setup_camera)
    .init_state::<GameState>()
    .add_plugins((splash::SplashPlugin, menu::MenuPlugin, game::GamePlugin))
    .run();
}
