use bevy::{prelude::*, window::PresentMode, ecs::schedule::ScheduleLabel};

mod game;
mod menu;
mod utils;
mod splash;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, ScheduleLabel)]
pub enum GameState {
    #[default]
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
            primary_window: Some(Window {
                title: "Simon".to_string(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        })
    )
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .add_startup_system(setup_camera)
    .add_state::<GameState>()
    .add_plugin(splash::SplashPlugin)
    .add_plugin(menu::MenuPlugin)
    .add_plugin(game::GamePlugin)
    .run();
}
