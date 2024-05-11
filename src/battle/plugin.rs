use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy::render::{
  camera::ScalingMode,
  view::RenderLayers
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_xpbd_3d::prelude::{
  PhysicsPlugins, 
  RigidBody, 
  Collider, Friction,
};
use bevy_xpbd_3d::resources::Gravity;
use rand::Rng;
use crate::states::GameState;
use crate::camera::battle_camera::{
  BattleCamera, 
  spawn_battle_camera, 
  despawn_battle_camera, 
  compute_fov
};
use crate::constants::{ 
  WIDTH, 
  HEIGHT, 
  WALL_SIZE, 
  GRAVITY_ACCELERATION,
  DEFAULT_CAMERA_DISTANCE, 
  MAX_CAMERA_DISTANCE, MAX_DICE_COUNT
};
use rand_distr::{
  Normal, 
  Distribution
};

use super::dice::{RespawnDicesEvent, DicePlugin, DiceFaceChangedEvent, FaceDescription, ActionType};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(DicePlugin)
      .add_systems(OnEnter(GameState::Battle), (spawn_battle_camera, add_battle_scene))
      .add_systems(OnExit(GameState::Battle), (despawn_battle_camera, despawn_battle_scene))
      .add_systems(Update, (debug_control, swap_camera).run_if(in_state(GameState::Battle)))
      .add_plugins(PhysicsPlugins::default())
      .init_resource::<CameraMode>()
      .insert_resource(Gravity(Vec3::NEG_Y * GRAVITY_ACCELERATION));
  }
}

#[derive(Component)]
struct BattleComponent;

fn add_battle_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn((SpatialBundle::default(), BattleComponent)).with_children(|commands| {
    let cube_mesh = meshes.add(Cuboid::default());

    // base
    commands.spawn((
      PbrBundle {
        mesh: cube_mesh.clone(),
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(256.0, 0.01, 144.0)),
        ..default()
      },
      RigidBody::Static,
      Friction::new(0.9),
      Collider::cuboid(1.0, 1.0, 1.0),
    ));
    // middle wall
    commands.spawn((
      PbrBundle {
        mesh: cube_mesh.clone(),
        material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(WIDTH, WALL_SIZE, 1.0)),
        ..default()
      },
      RigidBody::Static,
      Collider::cuboid(1.0, 1.0, 1.0),
    ));
    // north wall
    commands.spawn((
      PbrBundle {
        mesh: cube_mesh.clone(),
        material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0)),
        transform: Transform::from_xyz(0.0, 0.0, HEIGHT / 2.0).with_scale(Vec3::new(WIDTH, WALL_SIZE, 0.01)),
        ..default()
      },
      RigidBody::Static,
      Collider::cuboid(1.0, 1.0, 1.0),
    ));
    // south wall
    commands.spawn((
      PbrBundle {
        mesh: cube_mesh.clone(),
        material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0)),
        transform: Transform::from_xyz(0.0, 0.0, -HEIGHT / 2.0).with_scale(Vec3::new(WIDTH, WALL_SIZE, 0.01)),
        ..default()
      },
      RigidBody::Static,
      Collider::cuboid(1.0, 1.0, 1.0),
    ));
    // east wall
    commands.spawn((
      PbrBundle {
        mesh: cube_mesh.clone(),
        material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0)),
        transform: Transform::from_xyz(WIDTH / 2.0, 0.0, 0.0).with_scale(Vec3::new(0.01, WALL_SIZE, HEIGHT)),
        ..default()
      },
      RigidBody::Static,
      Collider::cuboid(1.0, 1.0, 1.0),
    ));
    // west wall
    commands.spawn((
      PbrBundle {
        mesh: cube_mesh.clone(),
        material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0)),
        transform: Transform::from_xyz(-WIDTH / 2.0, 0.0, 0.0).with_scale(Vec3::new(0.01, WALL_SIZE, HEIGHT)),
        ..default()
      },
      RigidBody::Static,
      Collider::cuboid(1.0, 1.0, 1.0),
    ));
    // light
    commands.spawn(
      DirectionalLightBundle {
        directional_light: DirectionalLight {
          illuminance: light_consts::lux::OVERCAST_DAY,
          shadows_enabled: true,
          ..default()
        },
        transform: Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
      }
    );
  });
}

fn despawn_battle_scene(
  mut commands: Commands,
  scene_: Query<Entity, With<BattleComponent>>,
) {
  let scene = scene_.single();
  commands.entity(scene).despawn_recursive();
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
          ..default()
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

// For debug
fn debug_control(
  mouse_buttons: Res<ButtonInput<MouseButton>>,
  mut respawn_dices: EventWriter<RespawnDicesEvent>,
  mut change_dice_face: EventWriter<DiceFaceChangedEvent>,
) {
  if mouse_buttons.just_pressed(MouseButton::Right) {
    for team_id in [0, 1] {
      for dice_id in 0..MAX_DICE_COUNT {
        for face_id in 0..6 {
          change_dice_face.send(DiceFaceChangedEvent {
            team_id: team_id,
            dice_id: dice_id as u32,
            face_id: face_id,
            face: FaceDescription {
              action_type: match rand::thread_rng().gen_range(0..3) {
                0 => ActionType::Attack,
                1 => ActionType::Defend,
                _ => ActionType::Heal,
              },
              pips_count: 0,
            }
          });
        }
      }
    }
  }

  if mouse_buttons.just_pressed(MouseButton::Left) {
    respawn_dices.send(RespawnDicesEvent {});
  }
}
