use bevy::prelude::*;

#[derive(Component)]
pub struct BattleCamera;

pub fn spawn_battle_camera(
  mut commands: Commands,
) {
  commands.spawn((
    Camera3dBundle::from_square(
      Vec3::ZERO, 
      Vec3::Z, 
      Vec3::new(0.0, 100.0, 0.0), 
      144.0,
    ),
    BattleCamera,
  ));
}

pub fn despawn_battle_camera(
  mut commands: Commands,
  camera_: Query<Entity, With<BattleCamera>>,
) {
  let camera = camera_.single();
  commands.entity(camera).despawn();
}

trait FromSquare {
  fn from_square(looking_at: Vec3, up: Vec3, looking_from: Vec3, square_size: f32) -> Self;
}

impl FromSquare for Camera3dBundle {
  fn from_square(looking_at: Vec3, up: Vec3, looking_from: Vec3, square_size: f32) -> Self {
    return Camera3dBundle {
      transform: Transform::from_translation(looking_from).looking_at(looking_at, up),
      projection: PerspectiveProjection {
        fov: compute_fov(looking_from.distance(looking_at), square_size),
        ..default()
      }.into(),
      ..default()
    };
  }
}

fn compute_fov(distance: f32, square_size: f32) -> f32 {
  return 2.0 * ((square_size/(2.0*distance)).atan());
}
