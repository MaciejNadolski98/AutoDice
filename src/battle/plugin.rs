use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_xpbd_3d::math::PI;
use bevy_xpbd_3d::prelude::{PhysicsPlugins, RigidBody, Collider, LinearVelocity, AngularVelocity, Friction};
use bevy_xpbd_3d::resources::Gravity;
use crate::states::GameState;
use crate::camera::battle_camera::{ BattleCamera, spawn_battle_camera, despawn_battle_camera, compute_fov };
use crate::constants::{ WIDTH, HEIGHT, WALL_SIZE, GRAVITY_ACCELERATION, DICE_SIZE, DEFAULT_CAMERA_DISTANCE, MAX_CAMERA_DISTANCE };
use rand_distr::{Normal, Distribution};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Battle), (spawn_battle_camera, add_battle_scene))
      .add_systems(OnExit(GameState::Battle), (despawn_battle_camera, despawn_battle_scene))
      .add_systems(Update, (debug_control, reset_dices, swap_camera).run_if(in_state(GameState::Battle)))
      .add_plugins(PhysicsPlugins::default())
      .init_resource::<CameraMode>()
      .insert_resource(Gravity(Vec3::NEG_Y * GRAVITY_ACCELERATION));
  }
}

#[derive(Component)]
enum Dice {
  BLUE,
  RED,
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

fn reset_dices(
  dices: Query<Entity, With<Dice>>,
  mut commands: Commands,
  mouse_buttons: Res<ButtonInput<MouseButton>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  if mouse_buttons.just_pressed(MouseButton::Left) {
    for dice_entity in & dices {
      commands.entity(dice_entity).despawn();
    }

    let cube_mesh = meshes.add(Cuboid::default());
    let dice_positions_red = [
      Vec3::new((-WIDTH + DICE_SIZE) / 2.0, DICE_SIZE, HEIGHT / 4.0),
      Vec3::new((-WIDTH + DICE_SIZE) / 2.0, DICE_SIZE, HEIGHT / 4.0 + DICE_SIZE * 1.1),
      Vec3::new((-WIDTH + DICE_SIZE) / 2.0, DICE_SIZE, HEIGHT / 4.0 - DICE_SIZE * 1.1),
      Vec3::new((-WIDTH + DICE_SIZE) / 2.0, DICE_SIZE * 2.2, HEIGHT / 4.0 + DICE_SIZE * 0.55),
      Vec3::new((-WIDTH + DICE_SIZE) / 2.0, DICE_SIZE * 2.2, HEIGHT / 4.0 - DICE_SIZE * 0.55),
    ];
    let dice_positions_blue = dice_positions_red.clone().map(|vec| {
      let mut ret = vec.clone();
      ret.x *= -1.0;
      ret.z *= -1.0;
      return ret;
    });
    // cubes
    for i in 0..5 {
      // blue
      commands.spawn((
        PbrBundle {
          mesh: cube_mesh.clone(),
          material: materials.add(Color::BLUE),
          transform: Transform::from_translation(dice_positions_blue[i])
            .with_scale(Vec3::new(DICE_SIZE, DICE_SIZE, DICE_SIZE)),
          ..default()
        },
        RigidBody::Dynamic,
        Friction::new(1.0),
        LinearVelocity::from(Vec3::new(
          1.0 * DICE_SIZE * (10.0 + random()), 
          (0.5 * DICE_SIZE * (10.0 + random())).clamp(0.0, 100.0), 
          10.0 * DICE_SIZE * random(),
        )),
        AngularVelocity::from(Vec3::new(
          random() * 2.0 * PI,
          random() * 2.0 * PI,
          random() * 2.0 * PI,
        )),
        Collider::cuboid(1.0, 1.0, 1.0),
        Dice::BLUE,
      ));

      // red
      commands.spawn((
        PbrBundle {
          mesh: cube_mesh.clone(),
          material: materials.add(Color::RED),
          transform: Transform::from_translation(dice_positions_red[i])
            .with_scale(Vec3::new(DICE_SIZE, DICE_SIZE, DICE_SIZE)),
          ..default()
        },
        RigidBody::Dynamic,
        Friction::new(1.0),
        LinearVelocity::from(Vec3::new(
          1.0 * DICE_SIZE * (-10.0 + random()),
          (0.5 * DICE_SIZE * (10.0 + random())).clamp(0.0, 100.0), 
          10.0 * DICE_SIZE * random(),
        )),
        AngularVelocity::from(Vec3::new(
          random() * 2.0 * PI,
          random() * 2.0 * PI,
          random() * 2.0 * PI,
        )),
        Collider::cuboid(1.0, 1.0, 1.0),
        Dice::RED,
      ));
    }
  }
}

fn random() -> f32 {
  let normal = Normal::<f32>::new(0.0, 1.0).unwrap();
  return normal.sample(&mut rand::thread_rng());
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
  keyboard_buttons: Res<ButtonInput<KeyCode>>,
  mut camera_: Query<(&mut Transform, &mut Projection), With<BattleCamera>>,
  mut commands: Commands,
  mut camera_mode_: ResMut<CameraMode>,
) {
  let camera_mode = *camera_mode_;
  if mouse_buttons.just_pressed(MouseButton::Right) {
    commands.spawn((
      NodeBundle::default(),
      CameraSwapTimer {
        timer: Timer::from_seconds(0.5, TimerMode::Once),
        to_isometric: match camera_mode {
          CameraMode::Isometric => false,
          CameraMode::Perspective => true,
        },
      },
    ));
    *camera_mode_ = match camera_mode {
      CameraMode::Isometric => CameraMode::Perspective,
      CameraMode::Perspective => CameraMode::Isometric,
    }
  }

  let (mut camera_transform, _) = camera_.single_mut();
  if keyboard_buttons.pressed(KeyCode::KeyW) {
    camera_transform.translation += Vec3::new(0.0, 0.0, 3.0);
  }
  if keyboard_buttons.pressed(KeyCode::KeyA) {
    camera_transform.translation += Vec3::new(3.0, 0.0, 0.0);
  }
  if keyboard_buttons.pressed(KeyCode::KeyS) {
    camera_transform.translation += Vec3::new(0.0, 0.0, -3.0);
  }
  if keyboard_buttons.pressed(KeyCode::KeyD) {
    camera_transform.translation += Vec3::new(-3.0, 0.0, 0.0);
  }
}
