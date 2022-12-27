use bevy::prelude::*;

use crate::utils;

#[derive(Component)]
pub struct Score;

pub fn spawn_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                size: Size::new(Val::Percent(100.0), Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Score)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Score",TextStyle {
                    font: asset_server.load("fonts/poppins/Poppins-Light.ttf"),
                    font_size: 80.0,
                    color: Color::WHITE,
                    ..Default::default()
                }),                
                ..default()
            });
            let gear_button = utils::build_icons(asset_server, "gear".to_string());
            parent.spawn(gear_button);
            });
        }