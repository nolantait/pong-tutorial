use bevy::prelude::*;

use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};

#[derive(Component, Default)]
#[require(Transform)]
struct Position(pub Vec2);

#[derive(Component, Default)]
struct Collider(Rectangle);

impl Collider {
  fn half_size(&self) -> Vec2 {
    self.0.half_size
  }
}

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component)]
#[require(
  Position,
  Velocity = Velocity(Vec2::new(1., 0.)),
  Collider = Collider(Rectangle::new(BALL_SIZE, BALL_SIZE))
)]
struct Ball;

#[derive(Component)]
#[require(
  Position,
  Collider = Collider(PADDLE_SHAPE)
)]
struct Paddle;

#[derive(Component)]
#[require(Position, Collider)]
struct Gutter;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
  Left,
  Right,
  Top,
  Bottom,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

fn spawn_camera(mut commands: Commands) {
  commands.spawn(Camera2d);
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
  for (mut transform, position) in &mut positionables {
    // Extend is going to turn this from a Vec2 to a Vec3
    transform.translation = position.0.extend(0.);
  }
}

const BALL_SIZE: f32 = 5.;
const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
const BALL_SPEED: f32 = 2.;
const BALL_COLOR: Color = Color::srgb(1., 0., 0.);

fn spawn_ball(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  // `Assets::add` will load these into memory and return a `Handle` (an ID)
  // to these assets. When all references to this `Handle` are cleaned up
  // the asset is cleaned up.
  let mesh = meshes.add(BALL_SHAPE);
  let material = materials.add(BALL_COLOR);

  // Here we are using `spawn` instead of `spawn_empty` followed by an
  // `insert`. They mean the same thing, letting us spawn many components on a
  // new entity at once.
  commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

const PADDLE_SHAPE: Rectangle = Rectangle::new(10., 50.);
const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);

fn spawn_paddles(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  window: Single<&Window>,
) {
  let mesh = meshes.add(PADDLE_SHAPE);
  let material = materials.add(PADDLE_COLOR);
  let half_window_size = window.resolution.size() / 2.;
  let padding = 20.;

  let player_position = Vec2::new(-half_window_size.x + padding, 0.);

  commands.spawn((
    Player,
    Paddle,
    Mesh2d(mesh.clone()),
    MeshMaterial2d(material.clone()),
    Position(player_position),
  ));

  let ai_position = Vec2::new(half_window_size.x - padding, 0.);

  commands.spawn((
    Ai,
    Paddle,
    Mesh2d(mesh.clone()),
    MeshMaterial2d(material.clone()),
    Position(ai_position),
  ));
}

const GUTTER_COLOR: Color = Color::srgb(0., 0., 1.);
const GUTTER_HEIGHT: f32 = 20.;

fn spawn_gutters(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  window: Single<&Window>,
) {
  let material = materials.add(GUTTER_COLOR);
  let padding = 20.;

  let gutter_shape = Rectangle::new(window.resolution.width(), GUTTER_HEIGHT);
  let mesh = meshes.add(gutter_shape);

  let top_gutter_position =
    Vec2::new(0., window.resolution.height() / 2. - padding);

  commands.spawn((
    Gutter,
    Mesh2d(mesh.clone()),
    MeshMaterial2d(material.clone()),
    Position(top_gutter_position),
    Collider(gutter_shape)
  ));

  let bottom_gutter_position =
    Vec2::new(0., -window.resolution.height() / 2. + padding);

  commands.spawn((
    Gutter,
    Mesh2d(mesh.clone()),
    MeshMaterial2d(material.clone()),
    Position(bottom_gutter_position),
    Collider(gutter_shape)
  ));
}

fn move_ball(ball: Single<(&mut Position, &Velocity), With<Ball>>) {
  let (mut position, velocity) = ball.into_inner();
  position.0 += velocity.0 * BALL_SPEED;
}

fn collide_with_side(ball: Aabb2d, wall: Aabb2d) -> Option<Collision> {
  if !ball.intersects(&wall) {
    return None;
  }

  let closest_point = wall.closest_point(ball.center());
  let offset = ball.center() - closest_point;

  let side = if offset.x.abs() > offset.y.abs() {
    if offset.x < 0. {
      Collision::Left
    } else {
      Collision::Right
    }
  } else if offset.y > 0. {
    Collision::Top
  } else {
    Collision::Bottom
  };

  Some(side)
}

fn handle_collisions(
  ball: Single<(&mut Velocity, &Position, &Collider), With<Ball>>,
  other_things: Query<(&Position, &Collider), Without<Ball>>,
) {
  let (mut ball_velocity, ball_position, ball_collider) = ball.into_inner();

  for (other_position, other_collider) in &other_things {
    if let Some(collision) = collide_with_side(
      Aabb2d::new(ball_position.0, ball_collider.half_size()),
      Aabb2d::new(other_position.0, other_collider.half_size()),
    ) {
      match collision {
        Collision::Left => {
          ball_velocity.0.x *= -1.;
        }
        Collision::Right => {
          ball_velocity.0.x *= -1.;
        }
        Collision::Top => {
          ball_velocity.0.y *= -1.;
        }
        Collision::Bottom => {
          ball_velocity.0.y *= -1.;
        }
      }
    }
  }
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (spawn_ball, spawn_paddles, spawn_camera, spawn_gutters))
    .add_systems(
      FixedUpdate,
      (
        project_positions,
        move_ball.before(project_positions),
        handle_collisions.after(move_ball),
      ),
    )
    .run();
}
