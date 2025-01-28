use bevy::prelude::*;
use bevy_xpbd_3d::math::PI;
use bevy_xpbd_3d::prelude::*;
use rand_distr::{Normal, Distribution};

use crate::constants::{MAX_DICE_COUNT, WIDTH, DICE_SIZE, HEIGHT};

use super::dice_render::{
  get_uv_vertex,
  DiceFaceImage
};
use super::events::RespawnDicesEvent;

pub struct DiceInstancePlugin;

impl Plugin for DiceInstancePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (despawn_dices, spawn_dices).chain().run_if(on_event::<RespawnDicesEvent>()));
  }
}

#[derive(Component)]
enum Dice {
  Blue = 0,
  Red = 1,
}

fn spawn_dices(
  mut meshes: ResMut<Assets<Mesh>>,
  mut commands: Commands,
  mut materials: ResMut<Assets<StandardMaterial>>,
  dice_face_image: Res<DiceFaceImage>
) {
  info!("Spawning dices");
  
  let dice_meshes = [0, 1].map(|team_id| {
    (0..MAX_DICE_COUNT).map(|dice_id| {
      meshes.add(Cuboid::default().mesh().with_removed_attribute(Mesh::ATTRIBUTE_UV_0).with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![
        // Front
        get_uv_vertex(team_id, dice_id as u32, 0, 0),
        get_uv_vertex(team_id, dice_id as u32, 0, 1),
        get_uv_vertex(team_id, dice_id as u32, 0, 2),
        get_uv_vertex(team_id, dice_id as u32, 0, 3),
        // Back
        get_uv_vertex(team_id, dice_id as u32, 1, 0),
        get_uv_vertex(team_id, dice_id as u32, 1, 1),
        get_uv_vertex(team_id, dice_id as u32, 1, 2),
        get_uv_vertex(team_id, dice_id as u32, 1, 3),
        // Right
        get_uv_vertex(team_id, dice_id as u32, 2, 0),
        get_uv_vertex(team_id, dice_id as u32, 2, 1),
        get_uv_vertex(team_id, dice_id as u32, 2, 2),
        get_uv_vertex(team_id, dice_id as u32, 2, 3),
        // Left
        get_uv_vertex(team_id, dice_id as u32, 3, 0),
        get_uv_vertex(team_id, dice_id as u32, 3, 1),
        get_uv_vertex(team_id, dice_id as u32, 3, 2),
        get_uv_vertex(team_id, dice_id as u32, 3, 3),
        // Top
        get_uv_vertex(team_id, dice_id as u32, 4, 0),
        get_uv_vertex(team_id, dice_id as u32, 4, 1),
        get_uv_vertex(team_id, dice_id as u32, 4, 2),
        get_uv_vertex(team_id, dice_id as u32, 4, 3),
        // Bottom
        get_uv_vertex(team_id, dice_id as u32, 5, 0),
        get_uv_vertex(team_id, dice_id as u32, 5, 1),
        get_uv_vertex(team_id, dice_id as u32, 5, 2),
        get_uv_vertex(team_id, dice_id as u32, 5, 3),
      ]))
  }).collect::<Vec<_>>()});
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
  // dices
  for i in 0..MAX_DICE_COUNT {
    // blue
    commands.spawn((
      PbrBundle {
        mesh: dice_meshes[0][i].clone(),
        material: materials.add(StandardMaterial { base_color_texture: Some(dice_face_image.image.clone()), ..default()}),
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
      Dice::Blue,
    ));

    // red
    commands.spawn((
      PbrBundle {
        mesh: dice_meshes[1][i].clone(),
        material: materials.add(StandardMaterial { base_color_texture: Some(dice_face_image.image.clone()), ..default()}),
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
      Dice::Red,
    ));
  }
}

fn despawn_dices(
  mut commands: Commands,
  entities: Query<Entity, With<Dice>>,
) {
  info!("Despawning dices");
  for entity in &entities {
    commands.entity(entity).despawn();
  }
}

fn random() -> f32 {
  let normal = Normal::<f32>::new(0.0, 1.0).unwrap();
  return normal.sample(&mut rand::thread_rng());
}
