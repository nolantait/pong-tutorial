use bevy::{
  math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
  prelude::*,
};

const BALL_SPEED: f32 = 1.;
const BALL_SIZE: f32 = 5.;
const PADDLE_SPEED: f32 = 4.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 50.;
const GUTTER_HEIGHT: f32 = 96.;

#[derive(Component)]
struct PlayerScore;

#[derive(Component)]
struct AiScore;

#[derive(Resource, Default)]
struct Score {
  player: u32,
  ai: u32,
}

enum Scorer {
  Ai,
  Player,
}

#[derive(Event)]
struct Scored(Scorer);

#[derive(Component)]
#[require(
    Position,
    Velocity(|| Velocity(Vec2::new(-1., 1.))),
    Shape(|| Shape(Vec2::new(BALL_SIZE, BALL_SIZE))),
)]
struct Ball;

#[derive(Component)]
#[require(
    Position,
    Shape(|| Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
    Velocity
)]
struct Paddle;

#[derive(Component)]
#[require(Position, Shape)]
struct Gutter;

#[derive(Component, Default)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
struct Shape(Vec2);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
  Left,
  Right,
  Top,
  Bottom,
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<Score>()
    .add_event::<Scored>()
    .add_systems(
      Startup,
      (
        spawn_ball,
        spawn_camera,
        spawn_paddles,
        spawn_gutters,
        spawn_scoreboard,
      ),
    )
    .add_systems(
      Update,
      (
        move_ball,
        handle_player_input,
        detect_scoring,
        move_ai,
        reset_ball.after(detect_scoring),
        update_score.after(detect_scoring),
        update_scoreboard.after(update_score),
        move_paddles.after(handle_player_input),
        project_positions.after(move_ball),
        handle_collisions.after(move_ball),
      ),
    )
    .run();
}

fn move_ai(
  mut ai: Query<(&mut Velocity, &Position), With<Ai>>,
  ball: Query<&Position, With<Ball>>,
) {
  if let Ok((mut velocity, position)) = ai.get_single_mut() {
    if let Ok(ball_position) = ball.get_single() {
      let a_to_b = ball_position.0 - position.0;
      velocity.0.y = a_to_b.y.signum();
    }
  }
}

fn update_scoreboard(
  mut player_score: Query<&mut Text, With<PlayerScore>>,
  mut ai_score: Query<&mut Text, (With<AiScore>, Without<PlayerScore>)>,
  score: Res<Score>,
) {
  if score.is_changed() {
    if let Ok(mut player_score) = player_score.get_single_mut() {
      player_score.0 = score.player.to_string();
    }

    if let Ok(mut ai_score) = ai_score.get_single_mut() {
      ai_score.0 = score.ai.to_string();
    }
  }
}

fn spawn_scoreboard(mut commands: Commands) {
  commands.spawn((
    PlayerScore,
    Text::new("0"),
    TextFont {
      font_size: 72.0,
      ..default()
    },
    TextColor(Color::WHITE),
    TextLayout::new_with_justify(JustifyText::Center),
    Node {
      position_type: PositionType::Absolute,
      top: Val::Px(5.0),
      right: Val::Px(15.0),
      ..default()
    },
  ));

  commands.spawn((
    AiScore,
    Text::new("0"),
    TextFont {
      font_size: 72.0,
      ..default()
    },
    TextColor(Color::WHITE),
    TextLayout::new_with_justify(JustifyText::Center),
    Node {
      position_type: PositionType::Absolute,
      top: Val::Px(5.0),
      left: Val::Px(15.0),
      ..default()
    },
  ));
}

fn update_score(mut score: ResMut<Score>, mut events: EventReader<Scored>) {
  for event in events.read() {
    match event.0 {
      Scorer::Ai => score.ai += 1,
      Scorer::Player => score.player += 1,
    }
  }
}

fn detect_scoring(
  mut ball: Query<&mut Position, With<Ball>>,
  window: Query<&Window>,
  mut events: EventWriter<Scored>,
) {
  if let Ok(window) = window.get_single() {
    let window_width = window.resolution.width();

    if let Ok(ball) = ball.get_single_mut() {
      if ball.0.x > window_width / 2. {
        events.send(Scored(Scorer::Ai));
      } else if ball.0.x < -window_width / 2. {
        events.send(Scored(Scorer::Player));
      }
    }
  }
}

fn reset_ball(
  mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
  mut events: EventReader<Scored>,
) {
  for event in events.read() {
    if let Ok((mut position, mut velocity)) = ball.get_single_mut() {
      match event.0 {
        Scorer::Ai => {
          position.0 = Vec2::new(0., 0.);
          velocity.0 = Vec2::new(-1., 1.);
        }
        Scorer::Player => {
          position.0 = Vec2::new(0., 0.);
          velocity.0 = Vec2::new(1., 1.);
        }
      }
    }
  }
}

fn handle_player_input(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut paddle: Query<&mut Velocity, With<Player>>,
) {
  if let Ok(mut velocity) = paddle.get_single_mut() {
    if keyboard_input.pressed(KeyCode::ArrowUp) {
      velocity.0.y = 1.;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
      velocity.0.y = -1.;
    } else {
      velocity.0.y = 0.;
    }
  }
}

fn spawn_gutters(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  window: Query<&Window>,
) {
  if let Ok(window) = window.get_single() {
    let window_width = window.resolution.width();
    let window_height = window.resolution.height();

    // We take half the window height because the center of our screen
    // is (0, 0). The padding would be half the height of the gutter as its
    // origin is also center rather than top left
    let top_gutter_y = window_height / 2. - GUTTER_HEIGHT / 2.;
    let bottom_gutter_y = -window_height / 2. + GUTTER_HEIGHT / 2.;

    let shape = Rectangle::from_size(Vec2::new(window_width, GUTTER_HEIGHT));
    let color = Color::srgb(0., 0., 0.);

    // We can share these meshes between our gutters by cloning them
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    commands.spawn((
      Gutter,
      Shape(shape.size()),
      Position(Vec2::new(0., top_gutter_y)),
      Mesh2d(mesh_handle.clone()),
      MeshMaterial2d(material_handle.clone()),
    ));

    commands.spawn((
      Gutter,
      Shape(shape.size()),
      Position(Vec2::new(0., bottom_gutter_y)),
      Mesh2d(mesh_handle.clone()),
      MeshMaterial2d(material_handle.clone()),
    ));
  }
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
  for (mut transform, position) in &mut positionables {
    transform.translation = position.0.extend(0.);
  }
}

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
  if let Ok((mut position, velocity)) = ball.get_single_mut() {
    position.0 += velocity.0 * BALL_SPEED;
  }
}

fn move_paddles(
  mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
  window: Query<&Window>,
) {
  if let Ok(window) = window.get_single() {
    let window_height = window.resolution.height();

    for (mut position, velocity) in &mut paddle {
      let new_position = position.0 + velocity.0 * PADDLE_SPEED;
      if new_position.y.abs()
        < window_height / 2. - GUTTER_HEIGHT - PADDLE_HEIGHT / 2.
      {
        position.0 = new_position;
      }
    }
  }
}

// Returns `Some` if `ball` collides with `wall`. The returned `Collision` is the
// side of `wall` that `ball` hit.
fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
  if !ball.intersects(&wall) {
    return None;
  }

  let center = ball.center();
  let closest = wall.closest_point(ball.center());
  let offset = ball.center() - closest;

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
  mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
  other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
  if let Ok((mut ball_velocity, ball_position, ball_shape)) =
    ball.get_single_mut()
  {
    for (position, shape) in &other_things {
      let circle = Circle {
        radius: ball_shape.0.x,
      };
      if let Some(collision) = collide_with_side(
        BoundingCircle::new(ball_position.0, circle.radius),
        Aabb2d::new(position.0, shape.0 / 2.0),
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
}

fn spawn_paddles(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  window: Query<&Window>,
) {
  println!("Spawning paddles...");

  if let Ok(window) = window.get_single() {
    let window_width = window.resolution.width();
    let padding = 50.;
    let right_paddle_x = window_width / 2. - padding;
    let left_paddle_x = -window_width / 2. + padding;

    let shape = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);

    let mesh = meshes.add(shape);
    let player_color = materials.add(Color::srgb(0., 1., 0.));
    let ai_color = materials.add(Color::srgb(0., 0., 1.));

    commands.spawn((
      Player,
      Paddle,
      Shape(shape.size()),
      Position(Vec2::new(right_paddle_x, 0.)),
      Mesh2d(mesh.clone()),
      MeshMaterial2d(player_color.clone()),
    ));

    commands.spawn((
      Ai,
      Paddle,
      Position(Vec2::new(left_paddle_x, 0.)),
      Mesh2d(mesh.clone()),
      MeshMaterial2d(ai_color.clone()),
    ));
  }
}

fn spawn_ball(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  println!("Spawning ball...");

  let shape = Circle::new(BALL_SIZE);
  let color = Color::srgb(1., 0., 0.);

  // `Assets::add` will load these into memory and return a `Handle` (an ID)
  // to these assets. When all references to this `Handle` are cleaned up
  // the asset is cleaned up.
  let mesh = meshes.add(shape);
  let material = materials.add(color);

  // Here we are using `spawn` instead of `spawn_empty` followed by an
  // `insert`. They mean the same thing, letting us spawn many components on a
  // new entity at once.
  commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn_empty().insert(Camera2d);
}
