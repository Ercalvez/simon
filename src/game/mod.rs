use bevy::{prelude::*, ecs::schedule::ShouldRun};
use crate::{ utils};
use super::{GameState};

mod tile;
mod score;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum TurnState {
    Player,
    Cpu,
}
pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
    app
    .add_event::<score::NextTurnEvent>()
    .init_resource::<score::Score>()
    .add_state(TurnState::Cpu)
    .add_system_set(SystemSet::on_enter(GameState::Game)
        .with_system(tile::spawn_tiles)
        .with_system(score::spawn_score)
    )
    .add_system_set(SystemSet::on_enter(TurnState::Cpu)
        .with_system(score::move_to_next_turn)
        .with_system(score::next_turn)
        .with_system(score::update_score)
        .with_run_criteria(run_if_new_game))
    
    .add_system_set(SystemSet::on_update(GameState::Game)
        .with_system(tile::toggle_tile)
        .with_system(tile::reset_tile_color)
    )
    .add_system_set(SystemSet::on_exit(GameState::Splash)
        .with_system(utils::despawn_screen::<OnGameScreen>),
    );
    
    }
}

fn run_if_new_game(
    app_state: Res<State<GameState>>
) -> ShouldRun
{
    if *app_state.current() == GameState::Game {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

#[derive(Component)]
struct OnGameScreen;