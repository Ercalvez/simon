use std::f32::consts::PI;
use bevy::prelude::*;

const RED: Color = Color::rgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
const YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
const MAGENTA: Color = Color::rgb(1.0, 0.0, 1.0);
const CYAN: Color = Color::rgb(0.0, 1.0, 1.0);

#[derive(Component)]
pub struct Size{
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Size {
    pub fn contains(&self,  &position: &Vec2) -> bool {
        match position {
            position if    position.x > self.x + self.width 
                    || position.x < self.x - self.width 
                    || position.y > self.y + self.height 
                    || position.y < self.y - self.height => false,
            _ => true
        }
    }
}

#[derive(Component)]
pub struct Tile{
    pub color: Color,
    pub clicked: bool
}

pub fn spawn_tiles(windows: Res<Windows>,mut commands: Commands){
    let colors = [RED,GREEN,BLUE,YELLOW,MAGENTA,CYAN];
    let color_size = colors.len();
    let window = windows.get_primary().unwrap();
    let center_x = window.width()/ 2.0;
    let center_y = window.height()/ 2.0;
    for (index, col) in colors.into_iter().enumerate(){
        commands.spawn_bundle(SpriteBundle {
            sprite : Sprite{color: col,
            ..Default::default()
        },
        transform: Transform{
            scale: Vec3::new(window.width()/10.0, window.height()/10.0, 50.0),
            translation: Vec3::new(window.width()/3.0*(2.0*PI*(index as f32)/(color_size as f32)).cos(),window.height()/3.0*(2.0*PI*(index as f32)/(color_size as f32)).sin(),0.0),
            ..Default::default()
        },
        ..Default::default()}
        )
        .insert(Tile{
                color: col,
                clicked: false
            })
        .insert(Size {
            x: center_x + window.width()/3.0*(2.0*PI*(index as f32)/(color_size as f32)).cos(),
            y: center_y + window.height()/3.0*(2.0*PI*(index as f32)/(color_size as f32)).sin(),
            width: window.width()/10.0,
            height: window.height()/10.0
        });
    }
    
}

pub fn toggle_tile(mut query: Query<(&mut Sprite,&mut Tile)>){
    for (mut sprite, mut tile) in query.iter_mut(){
        if tile.clicked{
            sprite.color = Color::from(Vec4::from(tile.color));
            tile.clicked = false;
        }
    }
}
