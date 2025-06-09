use avian3d::math::PI;
use avian3d::prelude::RigidBodyDisabled;
use bevy::prelude::*;
use bevy_defer::{fetch, AccessError, AsyncAccess, AsyncWorld};

use crate::constants::{ANGULAR_SPEED, DICE_SIZE, FACE_NORMALS, HEIGHT, LINEAR_SPEED};
use super::dice_instance::DiceEntityMap;
use super::{Dice, DiceID};
use super::roll::get_face_id;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
  fn build(&self, _app: &mut App) {
  }
}

pub async fn _move_dice_to_middle(
  dice_id: DiceID,
) -> Result<(), AccessError> {
  let target_y = if dice_id.team_id == 0 { HEIGHT / 5.0 } else { -HEIGHT / 5.0 };
  let target_position = Vec3::new(0.0, target_y, DICE_SIZE / 2.0);
  move_dice(dice_id, target_position).await?;
  Ok(())
}

pub async fn move_dice_to_row(
  dice_id: DiceID,
) -> Result<(), AccessError> {
  let entity = get_dice_entity(dice_id).await?;
  let row_position = fetch!(entity, Dice).get(|dice| { dice.row_position() })?;
  let target_x = compute_target_x(row_position);
  let target_y = if dice_id.team_id == 0 { HEIGHT / 5.0 } else { -HEIGHT / 5.0 };
  let target_position = Vec3::new(target_x, target_y, DICE_SIZE / 2.0);
  move_dice(dice_id, target_position).await?;
  Ok(())
}

fn compute_target_x(row_position: usize) -> f32 {
  let row_start = -DICE_SIZE * 4.0;
  let position = row_start + (DICE_SIZE * 2.0 * row_position as f32);
  position
}

pub async fn move_dice(
  dice_id: DiceID,
  target_position: Vec3,
) -> Result<(), AccessError> {
  let entity = AsyncWorld.resource::<DiceEntityMap>().get(|map| map.0.get(&dice_id).copied().ok_or(AccessError::Custom("Invalid DiceID")))??;
  loop {
    let delta = AsyncWorld.resource::<Time>().get_mut(|time| time.delta_secs())?;
    
    let current_position = fetch!(entity, Transform).get(|transform| transform.translation)?;
    let linear_displacement = LINEAR_SPEED * delta;
    let linear_difference = target_position - current_position;
    if linear_difference.length() < linear_displacement {
      fetch!(entity, Transform).get_mut(|transform| transform.translation = target_position)?;
      return Ok(());
    } else {
      let new_position = current_position + linear_difference.normalize() * linear_displacement;
      fetch!(entity, Transform).get_mut(|transform| transform.translation = new_position)?;
    }

    AsyncWorld.yield_now().await;
  }
}

pub async fn orient_dice(
  dice_id: DiceID,
) -> Result<(), AccessError> {
  let entity = get_dice_entity(dice_id).await?;
  let target_rotation = compute_target_rotation(fetch!(entity, Transform).get(|transform| transform.rotation)?);
  loop {
    let delta = AsyncWorld.resource::<Time>().get_mut(|time| time.delta_secs())?;
    
    let current_rotation = fetch!(entity, Transform).get(|transform| transform.rotation)?;
    let angular_displacement = ANGULAR_SPEED * delta;
    let rhs_angular_difference = current_rotation.inverse() * target_rotation;
    let (mut axis, mut angle) = rhs_angular_difference.to_axis_angle();
    if angle > PI {
      // Normalize angle to be within [0, PI]
      angle = angle - 2.0 * PI;
      axis = -axis; // Reverse the axis if we normalize the angle
    }
    if angle.abs() < angular_displacement {
      fetch!(entity, Transform).get_mut(|transform| transform.rotation = target_rotation)?;
      return Ok(());
    } else {
      let rotation = Quat::from_axis_angle(axis, angular_displacement);
      fetch!(entity, Transform).get_mut(|transform| transform.rotation *= rotation)?;
    }

    AsyncWorld.yield_now().await;
  }
}

fn compute_target_rotation(current_rotation: Quat) -> Quat {
  let face_id = get_face_id(current_rotation);
  Quat::from_rotation_arc(FACE_NORMALS[face_id], Vec3::Z)
}

pub async fn spin_dice(
  dice_id: DiceID,
  mut duration: f32,
) -> Result<(), AccessError> {
  let entity = get_dice_entity(dice_id).await?;
  let mut finished = false;
  while !finished {
    let mut delta = AsyncWorld.resource::<Time>().get_mut(|time| time.delta_secs())?;
    if duration <= delta {
      delta = duration;
      finished = true;
    }
    duration -= delta;

    let angular_displacement = ANGULAR_SPEED * delta;
    fetch!(entity, Transform).get_mut(|t| t.rotate(Quat::from_axis_angle(Vec3::Z, -angular_displacement)))?;

    AsyncWorld.yield_now().await;
  }
  Ok(())
}

async fn get_dice_entity(
  dice_id: DiceID,
) -> Result<Entity, AccessError> {
  let entity_map = AsyncWorld.resource::<DiceEntityMap>();
  entity_map.get(|map| map.0.get(&dice_id).copied().ok_or(AccessError::Custom("Invalid DiceID")))?
}

pub fn remove_physics(
  mut commands: Commands,
  entities: Query<Entity, With<Dice>>,
) {
  for entity in &entities {
    commands.entity(entity).insert(RigidBodyDisabled);
  }
}

pub fn add_physics(
  mut commands: Commands,
  entities: Query<Entity, With<Dice>>,
) {
  for entity in &entities {
    commands.entity(entity).remove::<RigidBodyDisabled>();
  }
}
