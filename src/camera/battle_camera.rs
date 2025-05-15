use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{
  constants::{DEFAULT_CAMERA_DISTANCE, MAX_CAMERA_DISTANCE, WIDTH, HEIGHT}, 
  states::GameState
};

#[derive(Component)]
struct BattleCamera;

#[derive(Event)]
pub struct SwapBattleCamera;

pub struct BattleCameraPlugin;

impl Plugin for BattleCameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<SwapBattleCamera>()
      .add_systems(OnEnter(GameState::Battle), spawn_battle_camera)
      .add_systems(OnExit(GameState::Battle), despawn_battle_camera)
      .add_systems(Update, swap_camera.run_if(in_state(GameState::Battle)))
      .init_resource::<CameraMode>();
  }
}

fn spawn_battle_camera(
  mut commands: Commands,
) {
  commands.spawn((
    Camera3d::default(),
    Transform::from_translation(Vec3::new(0.0, DEFAULT_CAMERA_DISTANCE, 0.0)).looking_at(Vec3::ZERO, Vec3::Z),
    Projection::Perspective(PerspectiveProjection {
      fov: compute_fov(Vec3::new(0.0, DEFAULT_CAMERA_DISTANCE, 0.0).distance(Vec3::ZERO), 144.0),
      ..default()
    }),
    BattleCamera,
  ));
  info!("computed fov: {}", compute_fov(Vec3::new(0.0, DEFAULT_CAMERA_DISTANCE, 0.0).distance(Vec3::ZERO), 144.0));
}

fn despawn_battle_camera(
  mut commands: Commands,
  camera_: Query<Entity, With<BattleCamera>>,
) {
  let camera = camera_.single();
  commands.entity(camera).despawn();
}

fn compute_fov(distance: f32, square_size: f32) -> f32 {
  return 2.0 * ((square_size/(2.0*distance)).atan());
}

#[derive(Component)]
struct CameraSwapTimer {
  pub timer: Timer,
  pub to_isometric: bool,
}

fn swap_camera(
  mut camera: Query<(&mut Projection, &mut Transform), With<BattleCamera>>,
  mut timer: Query<(&mut CameraSwapTimer, Entity)>,
  time: Res<Time>,
  mut commands: Commands,
) {
  for (mut camera_swap_timer, entity) in &mut timer {
    camera_swap_timer.timer.tick(time.delta());

    let mut t = camera_swap_timer.timer.elapsed().as_nanos() as f32 / camera_swap_timer.timer.duration().as_nanos() as f32;
    if !camera_swap_timer.to_isometric {
      t = 1.0 - t;
    }

    // Exponential interpolation
    let distance = DEFAULT_CAMERA_DISTANCE * (t * (MAX_CAMERA_DISTANCE / DEFAULT_CAMERA_DISTANCE).ln()).exp();

    let (mut projection, mut transform) = camera.single_mut();

    transform.translation.y = distance;
    *projection = PerspectiveProjection {
      fov: compute_fov(distance, HEIGHT),
      ..default()
    }.into();

    if camera_swap_timer.timer.finished() {
      if camera_swap_timer.to_isometric {
        *projection = OrthographicProjection {
          far: 2.0 * MAX_CAMERA_DISTANCE,
          scaling_mode: ScalingMode::Fixed { width: WIDTH, height: HEIGHT },
          ..OrthographicProjection::default_3d()
        }.into();
      }

      commands.entity(entity).despawn();
    }
  }
}

#[derive(Resource, Clone, Copy, Default, PartialEq)]
enum CameraMode {
  Isometric,
  #[default]
  Perspective,
}
