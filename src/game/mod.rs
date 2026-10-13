use bevy::{prelude::*};
use super::{GameState};

mod tile;
mod score;
mod sequence;

/// Whose turn it is. Only exists while `GameState::Game` is active, and starts
/// with the CPU every time a game starts.
#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Game)]
pub enum TurnState {
    #[default]
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
    .add_message::<tile::TileClicked>()
    .add_sub_state::<TurnState>()
    .add_systems(
        OnEnter(GameState::Game),
        (
            start_game,
            tile::spawn_tiles,
            score::spawn_score
        ))
    .add_systems(OnEnter(TurnState::Cpu), sequence::start_cpu_turn)
    .add_systems(Update, sequence::play_sequence.run_if(in_state(TurnState::Cpu)))
    .add_systems(OnEnter(TurnState::Player), sequence::start_player_turn)
    .add_systems(Update, sequence::check_player_input.run_if(in_state(TurnState::Player)))
    .add_systems(
        Update,
        (
            tile::reset_tile_color,
            score::update_score
        ).run_if(in_state(GameState::Game)))
    .add_systems(OnExit(GameState::Game), end_game)
    ;

    }
}

fn start_game(mut score: ResMut<score::Score>, mut sequence: ResMut<sequence::Sequence>) {
    *score = score::Score::default();
    *sequence = sequence::Sequence::default();
}

fn end_game(score: Res<score::Score>) {
    info!("Game over, score: {}", score.score);
}
