use avian3d::prelude::*;
use futures_::future::{join, join_all};
use bevy::prelude::*;
use bevy_defer::{fetch, AccessError, AsyncAccess, AsyncWorld};
use rand_distr::{Distribution, Normal};

use crate::{camera::SwapBattleCamera, constants::{ANGULAR_VELOCITY_EPSILON, DICE_SIZE, FACE_NORMALS, HEIGHT, LINEAR_VELOCITY_EPSILON, MAX_DICE_COUNT, WIDTH}};

use super::{animation::{add_physics, get_dice_entity, move_dice_to_middle, move_dice_to_row, orient_dice, remove_physics}, dice_instance::Rows, Dice, DiceID};

pub struct RollPlugin;

impl Plugin for RollPlugin {
  fn build(&self, _app: &mut App) {
  }
}

pub async fn set_physics(on: bool) -> Result<(), AccessError> {
  if on {
    AsyncWorld.run_system_cached(add_physics)?;
  } else {
    AsyncWorld.run_system_cached(remove_physics)?;
  }
  Ok(())
}

pub async fn roll_dices() -> Result<(), AccessError> {
  set_physics(true).await?;
  AsyncWorld.run_system_cached(set_dice_roll_positions_and_velocities)?;
  wait_for_dices_to_stop().await.unwrap();
  set_physics(false).await?;

  AsyncWorld.run_system_cached(compute_row_positions)?;
  AsyncWorld.send_event(SwapBattleCamera)?;

  let (result1, result2) = join(move_dices_to_rows(), orient_dices()).await;
  result1?; result2?;
  Ok(())
}

fn set_dice_roll_positions_and_velocities(
  mut dices: Query<(&mut Transform, &mut LinearVelocity, &mut AngularVelocity, &Dice)>,
) {
  let dice_positions_team_1 = [
    Vec3::new((-WIDTH + DICE_SIZE * 1.5) / 2.0, HEIGHT / 4.0, DICE_SIZE * 1.5,),
    Vec3::new((-WIDTH + DICE_SIZE * 1.5) / 2.0, HEIGHT / 4.0 + DICE_SIZE * 3.0, DICE_SIZE * 1.5,),
    Vec3::new((-WIDTH + DICE_SIZE * 1.5) / 2.0, HEIGHT / 4.0 - DICE_SIZE * 3.0, DICE_SIZE * 1.5,),
    Vec3::new((-WIDTH + DICE_SIZE * 1.5) / 2.0, HEIGHT / 4.0 + DICE_SIZE * 1.5, DICE_SIZE * 1.5,),
    Vec3::new((-WIDTH + DICE_SIZE * 1.5) / 2.0, HEIGHT / 4.0 - DICE_SIZE * 1.5, DICE_SIZE * 1.5,),
  ];
  let dice_positions_team_2 = dice_positions_team_1.clone().map(|vec| {
    let mut ret = vec.clone();
    ret.x *= -1.0;
    ret.y *= -1.0;
    return ret;
  });

  for (mut transform, mut linear_velocity, mut angular_velocity, dice) in &mut dices {
    if dice.id().team_id == 0 {
      *transform = Transform::from_translation(dice_positions_team_1[dice.id().dice_id as usize]).with_scale(Vec3::new(DICE_SIZE, DICE_SIZE, DICE_SIZE));
      *linear_velocity = LinearVelocity::from(Vec3::new(
        random(30.0 * DICE_SIZE, 10.0 * DICE_SIZE),
        random(0.0, 5.0 * DICE_SIZE),
        random(10.0 * DICE_SIZE, 10.0 * DICE_SIZE),
      ));
    } else {
      *transform = Transform::from_translation(dice_positions_team_2[dice.id().dice_id as usize]).with_scale(Vec3::new(DICE_SIZE, DICE_SIZE, DICE_SIZE));
      *linear_velocity = LinearVelocity::from(Vec3::new(
        random(-30.0 * DICE_SIZE, 10.0 * DICE_SIZE),
        random(0.0, 5.0 * DICE_SIZE),
        random(10.0 * DICE_SIZE, 10.0 * DICE_SIZE),
      ));
    }
    *angular_velocity = AngularVelocity::from(Vec3::new(
        random(0.0, 20.0),
        random(0.0, 20.0),
        random(0.0, 20.0),
    ));
  }
}

pub async fn resolve_dices() -> Result<(), AccessError> {
  let rows = AsyncWorld.resource::<Rows>().cloned().unwrap();

  for i in 0..MAX_DICE_COUNT {
    async fn resolve(dice_id: DiceID) -> Result<(), AccessError> {
      move_dice_to_middle(dice_id).await?;
      let _ = resolve_dice(dice_id).await;
      move_dice_to_row(dice_id).await?;
      Ok(())
    }
    if rows.team1.len() > i {
      let _ = resolve(rows.team1[i]).await;
    }
    if rows.team2.len() > i {
      let _ = resolve(rows.team2[i]).await;
    }
  }
  Ok(())
}

async fn resolve_dice(dice_id: DiceID) -> Result<(), AccessError> {
  let entity = get_dice_entity(dice_id).await?;
  let face_id = fetch!(entity, Transform).get(|transform| get_face_id(transform.rotation))?;
  let face = fetch!(entity, Dice).get(|dice| dice.face(face_id))?;

  face.resolve(dice_id).await?;

  Ok(())
}

async fn move_dices_to_rows() -> Result<(), AccessError> {
  let mut tasks = vec![];
  AsyncWorld.query::<&Dice>().for_each(|dice| {
    tasks.push(move_dice_to_row(dice.id()));
  });
  join_all(tasks).await;
  Ok(())
}

async fn orient_dices() -> Result<(), AccessError> {
  let mut tasks = vec![];
  AsyncWorld.query::<&Dice>().for_each(|dice| {
    tasks.push(orient_dice(dice.id()));
  });
  join_all(tasks).await;
  Ok(())
}

async fn wait_for_dices_to_stop() -> Result<(), AccessError> {
  loop {
    if dices_stopped().await.unwrap() {
      break;
    }
    AsyncWorld.sleep(0.1).await;
  }
  Ok(())
}

async fn dices_stopped() -> Result<bool, AccessError> {
  let mut stopped = true;
  AsyncWorld.query::<(&LinearVelocity, &AngularVelocity)>().for_each(|(linear_velocity, angular_velocity)| {
    if linear_velocity.0.length() > LINEAR_VELOCITY_EPSILON || angular_velocity.0.length() > ANGULAR_VELOCITY_EPSILON {
      stopped = false;
    }
  });
  return Ok(stopped);
}

fn compute_row_positions(
  mut dices: Query<(Entity, &Transform, &mut Dice)>,
  mut rows: ResMut<Rows>,
) {
  let mut team1 = Vec::new();
  let mut team2 = Vec::new();
  for (entity, transform, dice) in &dices {
    if dice.id().team_id == 0 {
      team1.push((transform.translation.x, entity));
    } else {
      team2.push((transform.translation.x, entity));
    }
  }
  team1.sort_by(|(a, _), (b, _)| a.partial_cmp(&b).unwrap());
  team2.sort_by(|(a, _), (b, _)| a.partial_cmp(&b).unwrap());

  *rows = Rows::default();
  for (_, entity) in team1 {
    let (_, _, mut dice) = dices.get_mut(entity).unwrap();
    dice.set_row_position(rows.team1.len());
    rows.team1.push(dice.id());
  }
  for (_, entity) in team2 {
    let (_, _, mut dice) = dices.get_mut(entity).unwrap();
    dice.set_row_position(rows.team2.len());
    rows.team2.push(dice.id());
  }
}

pub fn get_face_id(rotation: Quat) -> usize {
  let mut face_id = 0;
  let mut max_dot = -1.0;
  for i in 0..6 {
    let dot = rotation.mul_vec3(FACE_NORMALS[i]).dot(Vec3::new(0.0, 0.0, 1.0));
    if dot > max_dot {
      max_dot = dot;
      face_id = i;
    }
  }
  return face_id;
}

fn random(mean: f32, std_dev: f32) -> f32 {
  let normal = Normal::<f32>::new(mean, std_dev).unwrap();
  return normal.sample(&mut rand::thread_rng());
}
