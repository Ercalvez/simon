use std::{f32::consts::PI, hash::Hash};
use bevy::{prelude::*, utils::HashSet};
const RED: Color = Color::rgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
const YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
const MAGENTA: Color = Color::rgb(1.0, 0.0, 1.0);
const CYAN: Color = Color::rgb(0.0, 1.0, 1.0);

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
        self.clicked.hash(state);
    }
}


#[derive(Default,Debug,Resource)]
pub struct TileSet {
    pub set: HashSet<Tile>
}


fn resize_tile(index: usize, number_of_tile: usize) -> Transform {
  Transform { 
    scale: Vec3::new(10.0, 10.0, 0.0),
    translation: Vec3::new(45.0+30.0*(2.0*PI*(index as f32)/(number_of_tile as f32)).cos(),45.0+30.0*(2.0*PI*(index as f32)/(number_of_tile as f32)).sin(),0.0),
    ..default() }
}

pub fn spawn_tiles(mut commands: Commands){
    let colors = [RED,GREEN,BLUE,YELLOW,MAGENTA,CYAN];
    let color_size = colors.len();
    for (index, col) in colors.into_iter().enumerate(){
        let transform = resize_tile(index, color_size);
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
                timer: Timer::from_seconds(0.8,TimerMode::Repeating)
        });
    }
    
}

pub fn toggle_tile(mut tile: &mut Tile, background_color: &mut BackgroundColor) {
    tile.clicked = true;
    *background_color = BackgroundColor::from(Color::WHITE);
}

pub fn interact_tile(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &mut Tile),Changed<Interaction>>) {
        for (interaction ,mut color, mut tile) in interaction_query.iter_mut() {
            if *interaction == Interaction::Clicked {
                toggle_tile(&mut tile,&mut color);
        }
    }
}

pub fn toggle_cpu_tiles(tile_set: ResMut<TileSet>,mut query: Query<(&mut Tile, &mut BackgroundColor)>) {
    println!("===========================================");
    for (mut tile,mut background_color) in query.iter_mut() {
        if tile_set.set.iter().any(|tile2| { tile2.index == tile.index}){
            println!("{:?}",tile.color);
            toggle_tile(&mut tile,&mut background_color);

        }
    }
}

pub fn reset_tile_color(mut tile_query: Query<(&mut BackgroundColor, &mut Tile)>,time: Res<Time>) {
    for (mut color, mut tile) in tile_query.iter_mut() {
        if tile.clicked {
            tile.timer.tick(time.delta());
        }
        if tile.timer.finished() {
            (*color).0 =tile.color;
            tile.clicked = false;
        }
    }
}