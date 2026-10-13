use bevy::prelude::*;
use rand::Rng;

use crate::GameState;

use super::{TurnState, score::Score, tile::{self, Tile, TileClicked}};

// Time between two tiles lit by the CPU, also used as the pause before a round.
const PLAYBACK_INTERVAL: f32 = 1.0;

/// Ordered tiles the player must repeat. A tile can appear several times.
#[derive(Default,Debug,Resource)]
pub struct Sequence {
    pub tiles: Vec<usize>,
    pub player_index: usize,
}

#[derive(Debug,Resource)]
pub struct Playback {
    pub index: usize,
    pub timer: Timer,
}

impl Default for Playback {
    fn default() -> Self {
        Playback {
            index: 0,
            timer: Timer::from_seconds(PLAYBACK_INTERVAL, TimerMode::Repeating),
        }
    }
}

pub fn start_cpu_turn(mut sequence: ResMut<Sequence>, mut playback: ResMut<Playback>) {
    let mut rng = rand::thread_rng();
    sequence.tiles.push(rng.gen_range(0..tile::TILE_COUNT));
    *playback = Playback::default();
}

pub fn play_sequence(
    sequence: Res<Sequence>,
    mut playback: ResMut<Playback>,
    mut tiles: Query<(&mut Tile, &mut BackgroundColor)>,
    mut next_turn: ResMut<NextState<TurnState>>,
    time: Res<Time>
) {
    if !playback.timer.tick(time.delta()).just_finished() {
        return;
    }
    match sequence.tiles.get(playback.index) {
        Some(&index) => {
            for (mut tile, mut background_color) in tiles.iter_mut() {
                if tile.index == index {
                    tile::light_tile(&mut tile, &mut background_color);
                }
            }
            playback.index += 1;
        }
        None => next_turn.set(TurnState::Player),
    }
}

pub fn start_player_turn(mut sequence: ResMut<Sequence>) {
    sequence.player_index = 0;
}

pub fn check_player_input(
    mut sequence: ResMut<Sequence>,
    mut score: ResMut<Score>,
    mut clicks: EventReader<TileClicked>,
    mut next_turn: ResMut<NextState<TurnState>>,
    mut next_game: ResMut<NextState<GameState>>
) {
    for click in clicks.iter() {
        if sequence.tiles.get(sequence.player_index) != Some(&click.0) {
            next_game.set(GameState::Menu);
            break;
        }
        sequence.player_index += 1;
        if sequence.player_index == sequence.tiles.len() {
            score.score += 1;
            next_turn.set(TurnState::Cpu);
            break;
        }
    }
    // Drop clicks left after the turn ended so they don't count in the next one.
    clicks.clear();
}
