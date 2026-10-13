use bevy::{prelude::*};
use crate::{ utils};
use super::{GameState};

mod tile;
mod score;
mod sequence;

#[derive(Default,Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum TurnState {
    #[default]
    Idle,
    Cpu,
    Player,
}
pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
    app
    .init_resource::<score::Score>()
    .init_resource::<sequence::Sequence>()
    .init_resource::<sequence::Playback>()
    .add_event::<tile::TileClicked>()
    .add_state::<TurnState>()
    .add_systems(
        (
            start_game,
            tile::spawn_tiles,
            score::spawn_score
        ).in_schedule(OnEnter(GameState::Game)))
    .add_system(sequence::start_cpu_turn.in_schedule(OnEnter(TurnState::Cpu)))
    .add_system(sequence::play_sequence.in_set(OnUpdate(TurnState::Cpu)))
    .add_system(sequence::start_player_turn.in_schedule(OnEnter(TurnState::Player)))
    .add_systems(
        (
            tile::interact_tile,
            sequence::check_player_input
        ).chain().in_set(OnUpdate(TurnState::Player)))
    .add_systems(
        (
            tile::reset_tile_color,
            score::update_score
        ).in_set(OnUpdate(GameState::Game)))
    .add_systems(
        (
            end_game,
            utils::despawn_screen::<OnGameScreen>
        ).in_schedule(OnExit(GameState::Game)))
    ;

    }
}

fn start_game(
    mut score: ResMut<score::Score>,
    mut sequence: ResMut<sequence::Sequence>,
    mut next_turn: ResMut<NextState<TurnState>>
) {
    *score = score::Score::default();
    *sequence = sequence::Sequence::default();
    next_turn.set(TurnState::Cpu);
}

fn end_game(score: Res<score::Score>, mut next_turn: ResMut<NextState<TurnState>>) {
    info!("Game over, score: {}", score.score);
    next_turn.set(TurnState::Idle);
}

#[derive(Component)]
struct OnGameScreen;
