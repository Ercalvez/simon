use bevy::{prelude::*, core::FixedTimestep};

mod game;
mod menu;
mod utils;
mod splash;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Menu,
    Game,
}


fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    
}



fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        title: "Simon".to_string(),
        width: 1000.0,
        height: 1000.0,
        vsync: true,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .add_startup_system(setup_camera)
    .add_state(GameState::Splash)
    .add_plugin(splash::SplashPlugin)
    .add_plugin(menu::MenuPlugin)
    .add_plugin(game::GamePlugin)
    .add_plugins(DefaultPlugins)
    .run();
}
