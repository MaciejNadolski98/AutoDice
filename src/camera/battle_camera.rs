use bevy::prelude::*;
use bevy::animation::{animated_field, AnimationTarget, AnimationTargetId, RepeatAnimation};

use crate::constants::CAMERA_SWAP_TIME;
use crate::{
  constants::{DEFAULT_CAMERA_DISTANCE, MAX_CAMERA_DISTANCE, HEIGHT},
  states::GameState
};

#[derive(Component)]
pub struct BattleCamera;

#[derive(Component)]
pub struct BattleOverlayCamera;

#[derive(Event)]
pub struct SwapBattleCamera;

pub struct BattleCameraPlugin;

impl Plugin for BattleCameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<SwapBattleCamera>()
      .add_systems(OnEnter(GameState::Battle), spawn_battle_camera)
      .add_systems(OnExit(GameState::Battle), despawn_battle_camera)
      .add_systems(Update, update_camera_state.run_if(in_state(GameState::Battle)))
      .add_systems(Update, swap_camera.run_if(on_event::<SwapBattleCamera>))
      .init_resource::<LocalResources>();
  }
}

#[derive(Resource, Default)]
struct LocalResources {
  animation_index: AnimationNodeIndex,
}

fn despawn_battle_camera(
  mut commands: Commands,
  camera_: Query<Entity, With<BattleCamera>>,
) {
  let camera = camera_.single().unwrap();
  commands.entity(camera).despawn();
}

fn compute_fov(distance: f32, square_size: f32) -> f32 {
  2.0 * ((square_size/(2.0*distance)).atan())
}

#[derive(Component, Reflect, Copy, Clone)]
struct CameraState {
  is_perspective: bool,
  distance: f32,
  fov: f32,
}

impl Default for CameraState {
  fn default() -> Self {
    Self {
      is_perspective: false,
      distance: DEFAULT_CAMERA_DISTANCE,
      fov: compute_fov(DEFAULT_CAMERA_DISTANCE, HEIGHT),
    }
  }
}

fn swap_camera(
  mut animation_player: Single<&mut AnimationPlayer, With<BattleCamera>>,
  resources: Res<LocalResources>,
) {
  animation_player.adjust_speeds(-1.0);
  let animation = animation_player.animation_mut(resources.animation_index).unwrap();
  if animation.is_finished() {
    animation.set_repeat(RepeatAnimation::Count(animation.completions() + 1));
  }
}

fn update_camera_state(
  mut battle_camera: Query<(&mut Projection, &mut Transform, &CameraState), With<BattleCamera>>,
) {
  let (mut projection, mut transform, camera_state) = battle_camera.single_mut().unwrap();
  *projection = Projection::Perspective(PerspectiveProjection { fov: camera_state.fov, ..default()});
  transform.translation.z = camera_state.distance;
}

fn spawn_battle_camera(
  mut animations: ResMut<Assets<AnimationClip>>,
  mut graphs: ResMut<Assets<AnimationGraph>>,
  mut resources: ResMut<LocalResources>,
  mut commands: Commands,
) {
  let distance_curve = FunctionCurve::new(Interval::new(0., CAMERA_SWAP_TIME).unwrap(), |t| { DEFAULT_CAMERA_DISTANCE * ((t / CAMERA_SWAP_TIME) * (MAX_CAMERA_DISTANCE / DEFAULT_CAMERA_DISTANCE).ln()).exp()});
  let fov_curve = distance_curve.clone().map(|distance| { compute_fov(distance, HEIGHT) });

  let battle_camera = Name::new("BattleCamera");
  let mut animation = AnimationClip::default();
  let target_id = AnimationTargetId::from_name(&battle_camera);
  animation.add_curve_to_target(
    target_id,
    AnimatableCurve::new(
      animated_field!(CameraState::distance),
      distance_curve,
    )
  );
  animation.add_curve_to_target(
    target_id,
    AnimatableCurve::new(
      animated_field!(CameraState::fov),
      fov_curve,
    )
  );

  let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));
  resources.animation_index = animation_index;
  let mut player = AnimationPlayer::default();
  player.play(animation_index);
  player.adjust_speeds(-1.0);

  let camera_entity = commands.spawn((
    battle_camera,
    Camera3d::default(),
    Camera {
      order: 0,
      ..default()
    },
    Transform::from_translation(Vec3::new(0.0, 0.0, DEFAULT_CAMERA_DISTANCE)).looking_at(Vec3::ZERO, Vec3::Y),
    Projection::Perspective(PerspectiveProjection {
      fov: compute_fov(Vec3::new(0.0, 0.0, DEFAULT_CAMERA_DISTANCE).distance(Vec3::ZERO), HEIGHT),
      far: 1.1 * MAX_CAMERA_DISTANCE,
      near: 0.5 * DEFAULT_CAMERA_DISTANCE,
      ..default()
    }),
    CameraState::default(),
    AnimationGraphHandle(graphs.add(graph)),
    player,
    BattleCamera,
  )).id();
  commands
    .entity(camera_entity)
    .insert(AnimationTarget {
      id: target_id,
      player: camera_entity,
    });
}
