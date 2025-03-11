use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowResolution};

#[derive(Component)]
struct LeftBar; // marker

#[derive(Component)]
struct RightBar; // marker

#[derive(Component)]
struct Ball; // marker

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
            PhysicsPlugins::default().with_length_unit(300.0),
        ))
        .add_systems(Startup, setup_entities)
        .add_systems(Update, movements)
        .run();
}

fn setup_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let ball_size = 25.0;
    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::new(ball_size))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Dynamic,
        Collider::circle(ball_size),
        GravityScale(0.0),
        Friction::new(0.0),
        Restitution::new(1.0),
        LinearVelocity(Vec2::new(500.0, 500.0)),
    ));

    commands.spawn((
        LeftBar,
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(20.0, 300.0)))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(-500.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(20.0, 300.0),
        GravityScale(0.0),
        Restitution::new(1.0),
        Friction::new(0.0),
    ));

    commands.spawn((
        RightBar,
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(20.0, 300.0)))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(500.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(20.0, 300.0),
        GravityScale(0.0),
        Restitution::new(1.0),
        Friction::new(0.0),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(1280.0, 50.0)))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -360.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(1280.0, 50.0),
        GravityScale(0.0),
        Restitution::new(1.0),
        Friction::new(0.0),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(1280.0, 50.0)))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 360.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(1280.0, 50.0),
        GravityScale(0.0),
        Restitution::new(1.0),
        Friction::new(0.0),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(50.0, 720.0)))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(-640.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(50.0, 720.0),
        GravityScale(0.0),
        Restitution::new(1.0),
        Friction::new(0.0),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(50.0, 720.0)))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(640.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(50.0, 720.0),
        GravityScale(0.0),
        Restitution::new(1.0),
        Friction::new(0.0),
    ));
}

fn movements(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut left_bars: Query<&mut Transform, (With<LeftBar>, Without<RightBar>)>,
    mut right_bars: Query<&mut Transform, (With<RightBar>, Without<LeftBar>)>,
) {
    let delta_time = time.delta_secs_f64() as f32;
    let speed = 500.0;

    for mut left_bar in &mut left_bars {
        if keyboard_input.pressed(KeyCode::KeyW) {
            left_bar.translation.y += speed * delta_time;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            left_bar.translation.y -= speed * delta_time;
        }
    }

    for mut right_bar in &mut right_bars {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            right_bar.translation.y += speed * delta_time;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            right_bar.translation.y -= speed * delta_time;
        }
    }
}
