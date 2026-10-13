use bevy::prelude::*;

use crate::utils;

use crate::GameState;

#[derive(Default,Resource)]
pub struct Score {
    pub score: u32,
}

#[derive(Component)]
pub struct ScoreDisplay;

pub fn spawn_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                width: Val::Percent(100.0),
                ..default()
            },
            DespawnOnExit(GameState::Game),
        ))
        .with_children(|parent| {
            parent.spawn((utils::text(&asset_server, "Score: 0"), ScoreDisplay));
            parent.spawn(utils::build_icons(&asset_server, "gear"));
            });
        }

pub fn update_score(score: Res<Score>,mut text_query: Query<&mut Text,With<ScoreDisplay>>) {
    if !score.is_changed() {
        return;
    }
    for mut text in text_query.iter_mut() {
        text.0 = format!("{}{}","Score: ",score.score);
    }
}