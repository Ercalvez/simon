use bevy::{prelude::*, window::PresentMode};

mod game;
mod menu;
mod utils;
mod splash;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Menu,
    Game,
}


fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(UiCameraConfig::default());
    
}



fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(
        WindowPlugin {
            window: WindowDescriptor {
                title: "Simon".to_string(),
                present_mode: PresentMode::Fifo,
                ..default()
            },
            ..default()
        })
    )
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .add_startup_system(setup_camera)
    .add_state(GameState::Splash)
    .add_plugin(splash::SplashPlugin)
    .add_plugin(menu::MenuPlugin)
    .add_plugin(game::GamePlugin)
    .run();
}
