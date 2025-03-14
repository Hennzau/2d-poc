use bevy::prelude::*;

use crate::{GameState, SelectLevelMode};

mod main;
use main::{MainMenuEntity, setup_main_menu};

mod settings;
use settings::{SettingsMenuEntity, setup_settings_menu};

mod credits;
use credits::{CreditsMenuEntity, setup_credits_menu};

mod victory;
use victory::{VictoryMenuEntity, setup_victory_menu};

mod select_level;
use select_level::{SelectLevelMenuEntity, setup_select_level_menu};

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

pub fn despawn_on_exit<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
