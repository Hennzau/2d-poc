use bevy::prelude::*;

use crate::{GameState, LevelID};

#[derive(Component)]
struct PlayEntity;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        for i in 0..LevelID::NUM_LEVELS {
            app.add_systems(OnExit(GameState::Playing(LevelID(i as u32))), despawn_play);
            app.add_systems(OnEnter(GameState::Playing(LevelID(i as u32))), setup_play);
        }
    }
}

fn setup_play(mut commands: Commands) {
    let mut screen = commands.spawn(PlayEntity);
}

fn despawn_play(mut commands: Commands, entities: Query<Entity, With<PlayEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
