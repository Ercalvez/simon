use bevy::{prelude::*};
use super::{GameState};
use crate::utils;
pub struct SplashPlugin;

impl Plugin for SplashPlugin{
    fn build(&self, app: &mut App) {
    app
    .add_systems(OnEnter(GameState::Splash), splash_setup)
    .add_systems(Update, input.run_if(in_state(GameState::Splash)));

    }
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("images/branding.png");
    // Display the logo
    commands
    // First create a `Node` for centering what we want to display
    .spawn((
        Node {
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        DespawnOnExit(GameState::Splash),
    ))
    .with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(500.0),
                ..default()
            },
            ImageNode::new(icon),
        ));

        parent.spawn(utils::text(&asset_server, "Press SPACE to start"));
    });
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>){
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Menu);
    }
}
