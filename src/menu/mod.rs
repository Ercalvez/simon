use bevy::{prelude::*, app::AppExit};
use crate::utils;

use super::{GameState};
use std::fmt;
pub struct MenuPlugin;

impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup))
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu_action))
        .add_system_set(
            SystemSet::on_exit(GameState::Menu)
                .with_system(utils::despawn_screen::<OnMenuScreen>),
        );
        }
}
#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
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
    // Display the logo
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                size: Size::new(Val::Percent(100.0), Val::Auto),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnMenuScreen)
        .with_children(|parent| {
            for button in [MenuAction::NewGame,MenuAction::Settings,MenuAction::Quit] {
                let button_text = button.to_string();
                parent.spawn(ButtonBundle {
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    style: Style {
                        border: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(   button)
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text::from_section(
                            button_text,
                            TextStyle {
                                font: asset_server.load("fonts/poppins/Poppins-Light.ttf"),
                                font_size: 80.0,
                                color: Color::WHITE,
                                ..Default::default()
                            }
                        ),
                        ..Default::default()
                    });
                });
        
        }
});          
    
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuAction::Quit => app_exit_events.send(AppExit),
                MenuAction::NewGame => {
                    game_state.set(GameState::Game).unwrap();
                }
                MenuAction::Settings => (),
            }
        }
    }
}
