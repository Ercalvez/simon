use bevy::prelude::*;

#[derive(Component)]
pub struct Icon {
    pub name: String
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn build_icons(asset_server: Res<AssetServer>,icon_name: String) -> ButtonBundle
{
    let icon = asset_server.load(format!("{}{}{}","icons/PNG/White/1x/",icon_name,".png"));
    ButtonBundle {
        image: UiImage { 
            texture: icon,
            ..Default::default() },
        style: Style {
            size: Size {
                height: Val::Px(50.0),
                width: Val::Px(50.0)
            },
            ..Default::default()
        },
        ..default()
    }
}