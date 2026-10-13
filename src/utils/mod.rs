use bevy::{prelude::*, text::FontSize, ui_widgets::Button};

/// White 80px Poppins Light text, the style used across the UI.
pub fn text(asset_server: &AssetServer, value: impl Into<String>) -> impl Bundle {
    (
        Text::new(value),
        TextFont {
            font: asset_server.load("fonts/poppins/Poppins-Light.ttf").into(),
            font_size: FontSize::Px(80.0),
            ..default()
        },
        TextColor(Color::WHITE),
    )
}

pub fn build_icons(asset_server: &AssetServer, icon_name: &str) -> impl Bundle
{
    let icon = asset_server.load(format!("{}{}{}","icons/PNG/White/1x/",icon_name,".png"));
    (
        Button,
        ImageNode::new(icon),
        Node {
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            ..default()
        },
    )
}
