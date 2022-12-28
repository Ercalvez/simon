use bevy::prelude::*;

use crate::utils;

use super::TurnState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Score>()
        .add_event::<NextTurnEvent>()
        .add_startup_system(spawn_score)
        .add_system(next_turn);
    }
}

pub struct NextTurnEvent(TurnState);

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

pub fn move_to_next_turn(keys: Res<Input<KeyCode>>,mut turn_state: ResMut<State<TurnState>>, mut ev_next_turn: EventWriter<NextTurnEvent>) {
    if keys.just_pressed(KeyCode::Space) {
        println!("Space key pressed!");
        let next_state = match turn_state.current() {
            TurnState::Cpu => TurnState::Player,
            TurnState::Player =>TurnState::Cpu
        };
        ev_next_turn.send(NextTurnEvent(next_state.clone()));
        turn_state.set(next_state).unwrap();
    }
}

pub fn next_turn(mut ev_next_turn: EventReader<NextTurnEvent>, mut score: ResMut<Score>) {
    for _ in ev_next_turn.iter() {
        score.score+= 1;
    }
}

pub fn update_score(score: Res<Score>,mut text_query: Query<&mut Text,With<ScoreDisplay>>) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}{}","Score: ",score.score);
    }
}