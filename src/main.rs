use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Velocity(Vec2);

fn move_ball(mut ball: Query<(&mut Transform, &Velocity), With<Ball>>) {
    if let Ok((mut transform, velocity)) = ball.get_single_mut() {
        transform.translation += velocity.0.extend(0.);
    }
}

fn collide_with_paddle(
    mut ball: Query<(&mut Velocity, &Transform), With<Ball>>,
    paddles: Query<(&Transform, Mesh), With<Paddle>>,
) {
}

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning paddles...");
    let mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(5., 50.))));
    let material = materials.add(Color::rgb(0., 1., 0.).into());

    commands.spawn((
        Paddle,
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material: material.into(),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
    ));
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball...");
    let mesh = meshes.add(Mesh::from(shape::Circle::new(5.)));
    let material = materials.add(Color::rgb(1., 0., 0.).into());

    commands.spawn((
        Ball,
        Velocity(Vec2::new(1., 0.)),
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material: material.into(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera, spawn_paddles))
        .add_systems(Update, move_ball)
        .run();
}
