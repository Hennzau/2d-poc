use bevy::prelude::*;

pub const RED: Color = Color::srgb(220.0 / 255.0, 20.0 / 255.0, 60.0 / 255.0);
pub const BLUE: Color = Color::srgb(106.0 / 255.0, 90.0 / 255.0, 205.0 / 255.0);
pub const GREEN: Color = Color::srgb(154.0 / 255.0, 205.0 / 255.0, 50.0 / 255.0);
pub const YELLOW: Color = Color::srgb(255.0 / 255.0, 215.0 / 255.0, 0.0 / 255.0);
pub const DARKGREY: Color = Color::srgb(105.0 / 255.0, 105.0 / 255.0, 105.0 / 255.0);
pub const DARKBLUE: Color = Color::srgb(2.0 / 255.0, 4.0 / 255.0, 55.0 / 255.0);
pub const IVORY: Color = Color::srgb(255.0 / 255.0, 255.0 / 255.0, 212.0 / 255.0);
pub const VOLKSWAGEN_TAUPE: Color = Color::srgb(140.0 / 255.0, 134.0 / 255.0, 128.0 / 255.0);
pub const BLACK: Color = Color::srgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0);

pub fn spawn_ui_box(
    commands: &mut Commands,
    top: Val,
    left: Val,
    border: UiRect,
    margin1: UiRect,
    margin2: UiRect,
    text: impl Into<String>,
    font: Handle<Font>,
    font_size: f32,
) -> Entity {
    commands
        .spawn((
            BackgroundColor(BLACK),
            BorderColor(IVORY),
            BorderRadius::all(Val::Px(5.0)),
            Node {
                position_type: PositionType::Absolute,
                top,
                left,
                border,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    BackgroundColor(DARKBLUE),
                    BorderColor(IVORY),
                    Node {
                        border,
                        margin: margin2,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new(text),
                    TextColor(IVORY),
                    TextFont {
                        font,
                        font_size,
                        ..default()
                    },
                    Node {
                        margin: margin1,
                        ..default()
                    },
                ));
        })
        .id()
}
