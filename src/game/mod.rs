use std::ops::RangeBounds;

use bevy::{prelude::*, core::FixedTimestep};
use crate::{ utils};
use super::{GameState};

mod tile;
pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
    app
    .add_system_set(SystemSet::on_enter(GameState::Game)
    .with_system(tile::spawn_tiles))
    .add_system_set(SystemSet::on_update(GameState::Game)
    .with_system(input)
    .with_system(tile::toggle_tile)
    .with_run_criteria(FixedTimestep::step(0.1)))
    .add_system_set(
        SystemSet::on_exit(GameState::Splash)
            .with_system(utils::despawn_screen::<OnGameScreen>),
    );
    
    }
}

#[derive(Component)]
struct OnGameScreen;

fn input(mouse_input: Res<Input<MouseButton>>, windows: Res<Windows>, mut query: Query<(&mut tile::Tile,&mut Sprite, &tile::Size)>) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.pressed(MouseButton::Left) {
        println!("click");
        if let Some(position) = win.cursor_position()
        {
            println!("cursor");
            for (mut tile, mut sprite, size) in query.iter_mut() {
                if size.contains(&position)
                {
                    sprite.color = Color::WHITE;
                    tile.clicked = true;
                }
            }
        }
    }
}