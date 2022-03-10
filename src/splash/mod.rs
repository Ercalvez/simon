use bevy::{prelude::*};
use super::{GameState};
use crate::utils;
pub struct SplashPlugin;

impl Plugin for SplashPlugin{
    fn build(&self, app: &mut App) {
    app
    .add_system_set(SystemSet::on_enter(GameState::Splash)
    .with_system(splash_setup))
    .add_system_set(SystemSet::on_update(GameState::Splash)
    .with_system(input))
    .add_system_set(
        SystemSet::on_exit(GameState::Splash)
            .with_system(utils::despawn_screen::<OnSplashScreen>),
    );
    
    }
}

#[derive(Component)]
struct OnSplashScreen;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("images/branding.png");
    // Display the logo
    commands
    // First create a `NodeBundle` for centering what we want to display
    .spawn_bundle(NodeBundle {
        style: Style {
            margin: Rect::all(Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::SpaceBetween,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..Default::default()
    })
    .insert(OnSplashScreen)
    .with_children(|parent| {
        parent.spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(500.0), Val::Auto),
                ..Default::default()
            },
            image: UiImage(icon),
            ..Default::default()
        });
        
    parent.spawn_bundle(TextBundle {
        style: Style {
            ..Default::default()
        },
        text: Text::with_section(
            "Press SPACE to start",
            TextStyle {
                font: asset_server.load("fonts/poppins/Poppins-Light.ttf"),
                font_size: 80.0,
                color: Color::WHITE,
                ..Default::default()
            },
            Default::default(),
        ),
        ..Default::default()
    });
    
    
    
});
}

fn input(keys: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>){
    if keys.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Menu).unwrap();
    }
}

