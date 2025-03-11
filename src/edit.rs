use bevy::prelude::*;

use crate::{GameState, LevelID};

#[derive(Component)]
struct EditEntity;

pub struct EditPlugin;

impl Plugin for EditPlugin {
    fn build(&self, app: &mut App) {
        for i in 0..LevelID::NUM_LEVELS {
            app.add_systems(OnExit(GameState::Editing(LevelID(i as u32))), despawn_edit);
            app.add_systems(OnEnter(GameState::Editing(LevelID(i as u32))), setup_edit);
        }
    }
}

fn setup_edit(mut commands: Commands) {
    let mut screen = commands.spawn(EditEntity);
}

fn despawn_edit(mut commands: Commands, entities: Query<Entity, With<EditEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
