use std::{f32::consts::PI, hash::Hash};
use bevy::{prelude::*, ui_widgets::Button};
use crate::GameState;
use super::TurnState;
const RED: Color = Color::srgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
const YELLOW: Color = Color::srgb(1.0, 1.0, 0.0);
const MAGENTA: Color = Color::srgb(1.0, 0.0, 1.0);
const CYAN: Color = Color::srgb(0.0, 1.0, 1.0);
const COLORS: [Color; 6] = [RED,GREEN,BLUE,YELLOW,MAGENTA,CYAN];
pub const TILE_COUNT: usize = 4;

#[derive(Component,Debug, Clone)]
pub struct Tile{
    pub index: usize,
    pub color: Color,
    pub clicked: bool,
    pub timer: Timer
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

/// Sent when the player clicks a tile, with the tile index.
#[derive(Message)]
pub struct TileClicked(pub usize);


fn resize_tile(index: usize, number_of_tile: usize) -> Transform {
  Transform {
    scale: Vec3::new(10.0, 10.0, 0.0),
    translation: Vec3::new(45.0+30.0*(2.0*PI*(index as f32)/(number_of_tile as f32)).cos(),45.0+30.0*(2.0*PI*(index as f32)/(number_of_tile as f32)).sin(),0.0),
    ..default() }
}

pub fn spawn_tiles(mut commands: Commands){
    for (index, col) in COLORS.into_iter().take(TILE_COUNT).enumerate(){
        let transform = resize_tile(index, TILE_COUNT);
        commands.spawn((
            Button,
            Node {
                left: Val::Percent(transform.translation.x),
                top: Val::Percent(transform.translation.y),
                position_type: PositionType::Absolute,
                height: Val::Percent(transform.scale.y),
                width: Val::Percent(transform.scale.x),
                ..default()
            },
            BackgroundColor(col),
            Tile{
                index,
                color: col,
                clicked: false,
                timer: Timer::from_seconds(0.8,TimerMode::Once)
            },
            DespawnOnExit(GameState::Game),
        ))
        .observe(interact_tile);
    }

}

pub fn light_tile(tile: &mut Tile, background_color: &mut BackgroundColor) {
    tile.clicked = true;
    tile.timer.reset();
    *background_color = BackgroundColor::from(Color::WHITE);
}

// Clicks only count during the player's turn.
fn interact_tile(
    click: On<PointerClick>,
    turn: Option<Res<State<TurnState>>>,
    mut tiles: Query<(&mut BackgroundColor, &mut Tile)>,
    mut clicks: MessageWriter<TileClicked>) {
        if !turn.is_some_and(|turn| *turn.get() == TurnState::Player) {
            return;
        }
        if let Ok((mut color, mut tile)) = tiles.get_mut(click.entity) {
            light_tile(&mut tile,&mut color);
            clicks.write(TileClicked(tile.index));
        }
}

pub fn reset_tile_color(mut tile_query: Query<(&mut BackgroundColor, &mut Tile)>,time: Res<Time>) {
    for (mut color, mut tile) in tile_query.iter_mut() {
        if !tile.clicked {
            continue;
        }
        if tile.timer.tick(time.delta()).is_finished() {
            color.0 = tile.color;
            tile.clicked = false;
        }
    }
}
