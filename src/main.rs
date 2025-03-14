use bevy::{prelude::*, window::WindowResolution};
use edit::EditPlugin;
use menu::MenuPlugin;
use play::PlayPlugin;

pub mod edit;
pub mod menu;
pub mod play;

pub mod core;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LevelID(u32);

impl LevelID {
    const NUM_LEVELS: u32 = 10;

    pub fn new(id: u32) -> Self {
        if id < Self::NUM_LEVELS {
            Self(id)
        } else {
            Self(0)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectLevelMode {
    Playing,
    Editing,
}

#[derive(Clone, Copy, Eq, Default, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    SettingsMenu,
    CreditsMenu,
    VictoryMenu,
    SelectLevelMenu(SelectLevelMode),
    Playing(LevelID),
    Editing(LevelID),
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        resizable: false,
                        resolution: WindowResolution::new(1280., 720.),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            MenuPlugin,
            PlayPlugin,
            EditPlugin,
        ))
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
