use bevy::prelude::*;

#[derive(Component)]
pub struct CreditsMenuEntity;

pub fn setup_credits_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut screen = commands.spawn((CreditsMenuEntity,));
}
