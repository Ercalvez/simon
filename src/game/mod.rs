use bevy::prelude::*;
use crate::{ utils};
use super::{GameState};

mod tile;
mod score;
pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
    app
    .add_system_set(SystemSet::on_enter(GameState::Game)
    .with_system(score::spawn_score)
    .with_system(tile::spawn_tiles)
)
.add_system_set(SystemSet::on_update(GameState::Game)
    .with_system(tile::toggle_tile)
    .with_system(tile::reset_tile_color))
    .add_system_set(
        SystemSet::on_exit(GameState::Splash)
            .with_system(utils::despawn_screen::<OnGameScreen>),
    );
    
    }
}

#[derive(Component)]
struct OnGameScreen;