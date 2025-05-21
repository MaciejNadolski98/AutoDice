use bevy::prelude::*;
use avian3d::prelude::*;
use crate::camera::BattleCamera;
use crate::dice::TossDicesEvent;
use crate::states::GameState;
use crate::constants::{ 
  DICE_SIZE, GRAVITY_ACCELERATION, HEIGHT, WALL_SIZE, WIDTH
};
use crate::dice::Dice;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Battle), add_battle_scene)
      .add_systems(OnExit(GameState::Battle), despawn_battle_scene)
      .add_systems(Update, debug_control.run_if(in_state(GameState::Battle)))
      .add_plugins(PhysicsPlugins::default())
      .insert_resource(Gravity(Vec3::NEG_Z * GRAVITY_ACCELERATION));
  }
}

#[derive(Component)]
struct BattleComponent;

fn add_battle_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
) {
  commands.spawn((Name::new("Battle Scene"), Visibility::default(), Transform::default(), BattleComponent)).with_children(|commands| {
    commands.spawn((SceneRoot(asset_server.load(
      GltfAssetLabel::Scene(0).from_asset("autodicetable.gltf"),
      )),
      Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));

    let cube_mesh = meshes.add(Cuboid::default());

    commands.spawn((
      Name::new("Base"),
      Mesh3d(cube_mesh.clone()),
      MeshMaterial3d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.0))),
      Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(WIDTH, HEIGHT, 0.01)),
      RigidBody::Static,
      Collider::cuboid(1.0, 1.0, 1.0),
    ));

    commands.spawn((
      Name::new("Middle wall"),
      Mesh3d(cube_mesh.clone()),
      MeshMaterial3d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.0))),
      Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(WIDTH, 1.0, WALL_SIZE)),
      RigidBody::Static,
      Collider::cuboid(1.0, 1.0, 1.0),
    ));

    commands.spawn((
      Name::new("North wall"),
      Mesh3d(cube_mesh.clone()),
      MeshMaterial3d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.0))),
      Transform::from_xyz(0.0, HEIGHT / 2.0, 0.0).with_scale(Vec3::new(WIDTH, 0.01, WALL_SIZE)),
      RigidBody::Static,
      Collider::cuboid(1.0, 1.0, 1.0),
    ));
    
    commands.spawn((
      Name::new("South wall"),
      Mesh3d(cube_mesh.clone()),
      MeshMaterial3d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.0))),
      Transform::from_xyz(0.0, -HEIGHT / 2.0, 0.0).with_scale(Vec3::new(WIDTH, 0.01, WALL_SIZE)),
      RigidBody::Static,
      Collider::cuboid(1.0, 1.0, 1.0),
    ));
    
    commands.spawn((
      Name::new("East wall"),
      Mesh3d(cube_mesh.clone()),
      MeshMaterial3d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.0))),
      Transform::from_xyz(WIDTH / 2.0, 0.0, 0.0).with_scale(Vec3::new(0.01, HEIGHT, WALL_SIZE)),
      RigidBody::Static,
      Collider::cuboid(1.0, 1.0, 1.0),
    ));
    
    commands.spawn((
      Name::new("West wall"),
      Mesh3d(cube_mesh.clone()),
      MeshMaterial3d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.0))),
      Transform::from_xyz(-WIDTH / 2.0, 0.0, 0.0).with_scale(Vec3::new(0.01, HEIGHT, WALL_SIZE)),
      RigidBody::Static,
      Collider::cuboid(1.0, 1.0, 1.0),
    ));
    
    commands.spawn((
      Name::new("Light source"),
      DirectionalLight {
        illuminance: light_consts::lux::OVERCAST_DAY,
        shadows_enabled: true,
        ..default()
      },
      Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
  });
}

fn despawn_battle_scene(
  mut commands: Commands,
  scene_: Query<Entity, With<BattleComponent>>,
) {
  let scene = scene_.single();
  commands.entity(scene).despawn_recursive();
}

// For debug
fn debug_control(
  keys: Res<ButtonInput<KeyCode>>,
  mut toss_dices: EventWriter<TossDicesEvent>,
  mut battle_camera: Query<&mut Transform, With<BattleCamera>>,
  mut dices: Query<&mut Dice>,
) {
  if keys.just_pressed(KeyCode::KeyQ) {
    toss_dices.send(TossDicesEvent {});
  }

  if keys.pressed(KeyCode::ArrowUp) {
    battle_camera.single_mut().rotate_local_x(0.1);
  }

  if keys.pressed(KeyCode::ArrowDown) {
    battle_camera.single_mut().rotate_local_x(-0.1);
  }

  if keys.pressed(KeyCode::ArrowRight) {
    battle_camera.single_mut().rotate_local_y(-0.1);
  }

  if keys.pressed(KeyCode::ArrowLeft) {
    battle_camera.single_mut().rotate_local_y(0.1);
  }

  if keys.pressed(KeyCode::KeyW) {
    let mut transform = battle_camera.single_mut();
    let rotation = transform.rotation;
    let translation = transform.translation;
    let up = rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0));
    transform.translation = translation.move_towards(translation + up, 0.1 * DICE_SIZE);
  }

  if keys.pressed(KeyCode::KeyS) {
    let mut transform = battle_camera.single_mut();
    let rotation = transform.rotation;
    let translation = transform.translation;
    let up = rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0));
    transform.translation = translation.move_towards(translation + up, -0.1 * DICE_SIZE);
  }

  if keys.pressed(KeyCode::KeyD) {
    let mut transform = battle_camera.single_mut();
    let rotation = transform.rotation;
    let translation = transform.translation;
    let right = rotation.mul_vec3(Vec3::new(1.0, 0.0, 0.0));
    transform.translation = translation.move_towards(translation + right, 0.1 * DICE_SIZE);
  }

  if keys.pressed(KeyCode::KeyA) {
    let mut transform = battle_camera.single_mut();
    let rotation = transform.rotation;
    let translation = transform.translation;
    let right = rotation.mul_vec3(Vec3::new(1.0, 0.0, 0.0));
    transform.translation = translation.move_towards(translation + right, -0.1 * DICE_SIZE);
  }

  if keys.pressed(KeyCode::KeyX) {
    let mut transform = battle_camera.single_mut();
    let rotation = transform.rotation;
    let translation = transform.translation;
    let forward = rotation.mul_vec3(Vec3::new(0.0, 0.0, 1.0));
    transform.translation = translation.move_towards(translation + forward, 0.1 * DICE_SIZE);
  }

  if keys.pressed(KeyCode::KeyZ) {
    let mut transform = battle_camera.single_mut();
    let rotation = transform.rotation;
    let translation = transform.translation;
    let forward = rotation.mul_vec3(Vec3::new(0.0, 0.0, 1.0));
    transform.translation = translation.move_towards(translation + forward, -0.1 * DICE_SIZE);
  }

  let mut digits_pressed = vec![];
  if keys.just_pressed(KeyCode::Digit1) {
    digits_pressed.push(1);
  }
  if keys.just_pressed(KeyCode::Digit2) {
    digits_pressed.push(2);
  }
  if keys.just_pressed(KeyCode::Digit3) {
    digits_pressed.push(3);
  }
  if keys.just_pressed(KeyCode::Digit4) {
    digits_pressed.push(4);
  }
  if keys.just_pressed(KeyCode::Digit5) {
    digits_pressed.push(5);
  }

  for digit in digits_pressed {
    for mut dice in dices.iter_mut() {
      if dice.id().team_id == 0 && dice.id().dice_id == digit {
        let current_hp = dice.current_hp();
        if current_hp > 0 {
          dice.set_current_hp(current_hp - 1);
        }
      }
    }
  }
}
