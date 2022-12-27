use std::{f32::consts::PI};
use bevy::prelude::*;

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
        commands.spawn(ButtonBundle {

            background_color: BackgroundColor::from(col),
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
                timer: Timer::from_seconds(0.2,TimerMode::Repeating)
        });
    }
    
}

pub fn toggle_tile(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &mut Tile),(Changed<Interaction>, With<Button>)>) {
        for (interaction ,mut color, mut tile) in interaction_query.iter_mut() {
            if *interaction == Interaction::Clicked {
            tile.clicked = true;
            *color = BackgroundColor::from(Color::WHITE);
        }
    }
}

pub fn reset_tile_color(mut interaction_query: Query<(&mut BackgroundColor, &mut Tile), With<Button>>,time: Res<Time>) {
    for (mut color, mut tile) in interaction_query.iter_mut() {
        if tile.clicked {
            tile.timer.tick(time.delta());
        }
        if tile.timer.finished() {
            *color = BackgroundColor::from(tile.color);
        }
    }
}