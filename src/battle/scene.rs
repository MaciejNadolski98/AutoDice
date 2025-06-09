use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{constants::{BASE_SCALE, GRAVITY_ACCELERATION, HEIGHT, WALL_SIZE, WIDTH}, states::GameState};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(PhysicsPlugins::default())
      .add_systems(OnEnter(GameState::Battle), spawn_battle_scene)
      .add_systems(OnExit(GameState::Battle), despawn_battle_scene)
      .insert_resource(Gravity(Vec3::NEG_Z * GRAVITY_ACCELERATION));
  }
}


#[derive(Component)]
struct BattleComponent;

fn spawn_battle_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
) {
  commands.spawn((Name::new("Battle Scene"), Visibility::default(), Transform::default(), BattleComponent)).with_children(|commands| {
    commands.spawn((SceneRoot(asset_server.load(
      GltfAssetLabel::Scene(0).from_asset("autodicetable.gltf"),
      )),
      Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
        .with_scale(Vec3::splat(BASE_SCALE)),
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
  let scene = scene_.single().unwrap();
  commands.entity(scene).despawn();
}
