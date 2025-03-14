use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct SelectLevelMenuEntity;

pub fn setup_select_level_menu(
    current_state: Res<State<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Get the mode to know where the transition should happen after selecting a level
    let select_level_mode = match current_state.get() {
        GameState::SelectLevelMenu(mode) => mode.clone(),
        _ => panic!("Unexpected game state"),
    };

    let mut screen = commands.spawn((SelectLevelMenuEntity,));
}
