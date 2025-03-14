use bevy::prelude::*;

#[derive(Component)]
pub struct VictoryMenuEntity;

pub fn setup_victory_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut screen = commands.spawn((VictoryMenuEntity,));
}
