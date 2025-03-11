use bevy::prelude::*;

use crate::{GameState, LevelID, SelectLevelMode};

#[derive(Component)]
struct MainMenuEntity;

#[derive(Component)]
struct SettingsMenuEntity;

#[derive(Component)]
struct CreditsMenuEntity;

#[derive(Component)]
struct VictoryMenuEntity;

#[derive(Component)]
struct SelectLevelMenuEntity;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnEnter(GameState::SettingsMenu), setup_settings_menu)
            .add_systems(OnEnter(GameState::CreditsMenu), setup_credits_menu)
            .add_systems(OnEnter(GameState::VictoryMenu), setup_victory_menu)
            .add_systems(
                OnEnter(GameState::SelectLevelMenu(SelectLevelMode::Playing)),
                setup_select_level_menu,
            )
            .add_systems(
                OnEnter(GameState::SelectLevelMenu(SelectLevelMode::Editing)),
                setup_select_level_menu,
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                despawn_on_exit::<MainMenuEntity>,
            )
            .add_systems(
                OnExit(GameState::SettingsMenu),
                despawn_on_exit::<SettingsMenuEntity>,
            )
            .add_systems(
                OnExit(GameState::CreditsMenu),
                despawn_on_exit::<CreditsMenuEntity>,
            )
            .add_systems(
                OnExit(GameState::VictoryMenu),
                despawn_on_exit::<VictoryMenuEntity>,
            )
            .add_systems(
                OnExit(GameState::SelectLevelMenu(SelectLevelMode::Playing)),
                despawn_on_exit::<SelectLevelMenuEntity>,
            )
            .add_systems(
                OnExit(GameState::SelectLevelMenu(SelectLevelMode::Editing)),
                despawn_on_exit::<SelectLevelMenuEntity>,
            );
    }
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut screen = commands.spawn((MainMenuEntity,));
}

fn setup_settings_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut screen = commands.spawn((SettingsMenuEntity,));
}

fn setup_credits_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut screen = commands.spawn((CreditsMenuEntity,));
}

fn setup_victory_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut screen = commands.spawn((VictoryMenuEntity,));
}

fn setup_select_level_menu(
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

pub fn despawn_on_exit<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
