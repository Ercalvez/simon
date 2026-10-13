use bevy::prelude::*;

use crate::utils;

use super::OnGameScreen;

#[derive(Default,Resource)]
pub struct Score {
    pub score: u32,
}

#[derive(Component)]
pub struct ScoreDisplay;

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
        .insert(OnGameScreen)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Score: 0",TextStyle {
                    font: asset_server.load("fonts/poppins/Poppins-Light.ttf"),
                    font_size: 80.0,
                    color: Color::WHITE,
                    ..Default::default()
                }),
                ..default()
            })
            .insert(ScoreDisplay);
            let gear_button = utils::build_icons(asset_server, "gear".to_string());
            parent.spawn(gear_button);
            });
        }

pub fn update_score(score: Res<Score>,mut text_query: Query<&mut Text,With<ScoreDisplay>>) {
    if !score.is_changed() {
        return;
    }
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}{}","Score: ",score.score);
    }
}