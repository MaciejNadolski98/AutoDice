use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use bevy_xpbd_3d::resources::Gravity;
use rand::Rng;
use crate::dice::{RespawnDicesEvent, DiceFaceChangedEvent, FaceDescription, ActionType};
use crate::states::GameState;
use crate::constants::{ 
  WIDTH, 
  HEIGHT, 
  WALL_SIZE, 
  GRAVITY_ACCELERATION,
  MAX_DICE_COUNT
};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Battle), add_battle_scene)
      .add_systems(OnExit(GameState::Battle), despawn_battle_scene)
      .add_systems(Update, debug_control.run_if(in_state(GameState::Battle)))
      .add_plugins(PhysicsPlugins::default())
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

// For debug
fn debug_control(
  mouse_buttons: Res<ButtonInput<MouseButton>>,
  mut respawn_dices: EventWriter<RespawnDicesEvent>,
  mut change_dice_face: EventWriter<DiceFaceChangedEvent>,
  mut game_state: ResMut<NextState<GameState>>,
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
              action_type: match rand::thread_rng().gen_range(0..4) {
                0 => ActionType::Attack,
                1 => ActionType::Defend,
                2 => ActionType::Heal,
                _ => ActionType::Fire,
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
