use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::utils;

use super::{TurnState, tile::{Tile, TileSet}};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Score>()
        .add_startup_system(spawn_score)
        .add_system(next_turn);
    }
}

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
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Score",TextStyle {
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

pub fn move_to_next_turn(mut commands: Commands, keys: Res<Input<KeyCode>>, turn_state: Res<State<TurnState>>) {
    if keys.just_pressed(KeyCode::Space) {
        let next_state = match turn_state.0 {
            TurnState::Cpu => TurnState::Player,
            TurnState::Player =>TurnState::Cpu
        };
        commands.insert_resource(NextState(Some(next_state)))
    }
}

pub fn next_turn(mut score: ResMut<Score>, mut tiles: Query<&mut Tile>, mut tile_vec: ResMut<TileSet> ) {
    let mut rng = rand::thread_rng();
    score.score+= 1;
    let  tile = tiles.iter_mut().choose(&mut rng).unwrap();
    tile_vec.set.insert(tile.clone());
        
}

pub fn update_score(score: Res<Score>,mut text_query: Query<&mut Text,With<ScoreDisplay>>) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}{}","Score: ",score.score);
    }
}