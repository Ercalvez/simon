use std::{f32::consts::PI, hash::Hash};
use bevy::prelude::*;
use super::OnGameScreen;
const RED: Color = Color::rgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
const YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
const MAGENTA: Color = Color::rgb(1.0, 0.0, 1.0);
const CYAN: Color = Color::rgb(0.0, 1.0, 1.0);
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
        let background_color = BackgroundColor::from(col);
        commands.spawn(ButtonBundle {

            background_color: background_color.clone(),
            style: Style {
                position: UiRect {
                    left: Val::Percent(transform.translation.x),
                    top: Val::Percent(transform.translation.y),
                    ..default()
                },
                position_type: PositionType::Absolute,
                size: Size {
                    height: Val::Percent(transform.scale.y),
                    width: Val::Percent(transform.scale.x)
                },
                ..Default::default()
            },
            ..default()
        }
        )
        .insert(Tile{
                index,
                color: col,
                clicked: false,
                timer: Timer::from_seconds(0.8,TimerMode::Once)
        })
        .insert(OnGameScreen);
    }
    
}

pub fn light_tile(tile: &mut Tile, background_color: &mut BackgroundColor) {
    tile.clicked = true;
    tile.timer.reset();
    *background_color = BackgroundColor::from(Color::WHITE);
}

pub fn interact_tile(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &mut Tile),Changed<Interaction>>,
    mut clicks: EventWriter<TileClicked>) {
        for (interaction ,mut color, mut tile) in interaction_query.iter_mut() {
            if *interaction == Interaction::Clicked {
                light_tile(&mut tile,&mut color);
                clicks.send(TileClicked(tile.index));
        }
    }
}

pub fn reset_tile_color(mut tile_query: Query<(&mut BackgroundColor, &mut Tile)>,time: Res<Time>) {
    for (mut color, mut tile) in tile_query.iter_mut() {
        if !tile.clicked {
            continue;
        }
        if tile.timer.tick(time.delta()).finished() {
            (*color).0 =tile.color;
            tile.clicked = false;
        }
    }
}
