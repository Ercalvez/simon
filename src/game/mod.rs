use bevy::{prelude::*};
use crate::{ utils};
use super::{GameState};

mod tile;
mod score;

#[derive(Default,Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum TurnState {
    Player,
    #[default]
    Cpu,
}
pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
    app
    .init_resource::<score::Score>()
    .init_resource::<tile::TileSet>()
    .add_state::<TurnState>()
    .add_systems(
        (
            tile::spawn_tiles,
            score::spawn_score
        ).in_schedule(OnEnter(GameState::Game)))
    .add_systems(
        (
            score::move_to_next_turn.run_if(run_if_new_game),
            score::next_turn.run_if(run_if_new_game),
            score::update_score.run_if(run_if_new_game),
            tile::toggle_cpu_tiles.run_if(run_if_new_game)
        ).in_schedule(OnEnter(TurnState::Cpu)))
    .add_systems(
        (
            tile::interact_tile,
            tile::reset_tile_color
        ).in_set(OnUpdate(GameState::Game)))
    .add_system(utils::despawn_screen::<OnGameScreen>.in_schedule(OnExit(GameState::Splash)))
    ;
    
    }
}

fn run_if_new_game(
    app_state: Res<State<GameState>>
) -> bool
{
    return app_state.0 == GameState::Game;
}

#[derive(Component)]
struct OnGameScreen;