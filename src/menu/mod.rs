use bevy::{prelude::*, ui_widgets::Button};
use crate::utils;

use super::{GameState};
use std::fmt;
pub struct MenuPlugin;

impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Menu), menu_setup);
        }
}

#[derive(Component, Clone, Copy)]
enum MenuAction {
    NewGame,
    Settings,
    Quit
}

impl fmt::Display for MenuAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            MenuAction::NewGame => write!(f, "New Game"),
            MenuAction::Settings => write!(f, "Settings"),
            MenuAction::Quit => write!(f, "Quit"),
        }

    }
}

fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                ..default()
            },
            DespawnOnExit(GameState::Menu),
        ))
        .with_children(|parent| {
            for button in [MenuAction::NewGame,MenuAction::Settings,MenuAction::Quit] {
                parent.spawn((
                    Button,
                    Node {
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    button,
                ))
                .observe(menu_action)
                .with_children(|parent| {
                    parent.spawn(utils::text(&asset_server, button.to_string()));
                });

        }
});

}

fn menu_action(
    click: On<PointerClick>,
    actions: Query<&MenuAction>,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit: MessageWriter<AppExit>
) {
    let Ok(action) = actions.get(click.entity) else {
        return;
    };
    match action {
        MenuAction::Quit => {
            app_exit.write(AppExit::Success);
        }
        MenuAction::NewGame => next_state.set(GameState::Game),
        MenuAction::Settings => (),
    }
}
