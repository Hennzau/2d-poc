use bevy::prelude::*;

#[derive(Component)]
pub struct SettingsMenuEntity;

pub fn setup_settings_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut screen = commands.spawn((SettingsMenuEntity,));
}
