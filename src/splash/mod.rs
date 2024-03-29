use bevy::{prelude::*};
use super::{GameState};
use crate::utils;
pub struct SplashPlugin;

impl Plugin for SplashPlugin{
    fn build(&self, app: &mut App) {
    app
    .add_system(splash_setup.in_schedule(OnEnter(GameState::Splash)))
    .add_system(input.in_set(OnUpdate(GameState::Splash)))
    .add_system(utils::despawn_screen::<OnSplashScreen>.in_schedule(OnExit(GameState::Splash))
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
    .spawn(NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..Default::default()
    })
    .insert(OnSplashScreen)
    .with_children(|parent| {
        parent.spawn(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(500.0), Val::Auto),
                ..Default::default()
            },
            image: UiImage::new(icon),
            ..Default::default()
        });
        
    parent.spawn(TextBundle {
        style: Style {
            ..Default::default()
        },
        text: Text::from_section(
            "Press SPACE to start",
            TextStyle {
                font: asset_server.load("fonts/poppins/Poppins-Light.ttf"),
                font_size: 80.0,
                color: Color::WHITE,
                ..Default::default()
            }
        ),
        ..Default::default()
    });
    
    
    
});
}

fn input(mut commands: Commands, keys: Res<Input<KeyCode>>){
    if keys.just_pressed(KeyCode::Space) {
        commands.insert_resource(NextState(Some(GameState::Menu)));
    }
}

