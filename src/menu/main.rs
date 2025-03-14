use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuEntity;

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title = crate::core::spawn_ui_box(
        &mut commands,
        Val::Px(180.0),
        Val::Auto,
        UiRect::all(Val::Px(2.0)),
        UiRect::all(Val::Px(10.0)),
        UiRect::all(Val::Px(15.0)),
        "Sprint The Game",
        asset_server.load("fonts/BulletTrace7-rppO.ttf"),
        60.0,
    );

    let play = crate::core::spawn_ui_box(
        &mut commands,
        Val::Auto,
        Val::Auto,
        UiRect::all(Val::Px(2.0)),
        UiRect::all(Val::Px(10.0)),
        UiRect::all(Val::Px(2.0)),
        "Play",
        asset_server.load("fonts/BulletTrace7-rppO.ttf"),
        30.0,
    );

    let settings = crate::core::spawn_ui_box(
        &mut commands,
        Val::Auto,
        Val::Auto,
        UiRect::all(Val::Px(2.0)),
        UiRect::all(Val::Px(10.0)),
        UiRect::all(Val::Px(2.0)),
        "Settings",
        asset_server.load("fonts/BulletTrace7-rppO.ttf"),
        30.0,
    );

    let mut actions = commands.spawn((Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        top: Val::Px(50.0),
        flex_direction: FlexDirection::Row,
        ..default()
    },));

    actions.add_children(&[settings, play]);

    let actions = actions.id();

    let background_image = asset_server.load("images/Sprint_Background.png");

    let mut screen = commands.spawn((
        MainMenuEntity,
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
    ));

    screen.with_child((
        Sprite {
            image: background_image.clone(),
            ..Default::default()
        },
        Transform::from_xyz(-640.0, -360.0, 0.0),
    ));

    screen.add_children(&[title, actions]);
}
