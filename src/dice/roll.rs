use avian3d::prelude::*;
use bevy::prelude::*;
use rand_distr::{Distribution, Normal};

use crate::constants::{ANGULAR_VELOCITY_EPSILON, DICE_SIZE, HEIGHT, LINEAR_VELOCITY_EPSILON, WIDTH};

use super::{events::{DicesStoppedEvent, RollResultEvent}, Dice, DiceID, TossDicesEvent};

pub struct RollPlugin;

#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub struct DicesRolling(pub bool);

impl Plugin for RollPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<DicesRolling>()
      .add_systems(Update, roll_dices.run_if(on_event::<TossDicesEvent>))
      .add_systems(Update, check_if_dices_stopped.run_if(resource_equals::<DicesRolling>(DicesRolling(true))))
      .add_systems(Update, check_roll_results.run_if(on_event::<DicesStoppedEvent>));
  }
}

fn roll_dices(
  mut dices: Query<(&mut Transform, &mut LinearVelocity, &mut AngularVelocity, &Dice)>,
  mut dices_rolling: ResMut<DicesRolling>,
) {
  dices_rolling.0 = true;
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

fn check_if_dices_stopped(
  dices: Query<(&LinearVelocity, &AngularVelocity), With<Dice>>,
  mut dices_rolling: ResMut<DicesRolling>,
  mut event_writer: EventWriter<DicesStoppedEvent>
) {
  for (linear_velocity, angular_velocity) in &dices {
    if linear_velocity.0.length() > LINEAR_VELOCITY_EPSILON || angular_velocity.0.length() > ANGULAR_VELOCITY_EPSILON {
      return;
    }
  }
  dices_rolling.0 = false;
  event_writer.send(DicesStoppedEvent);
}

fn check_roll_results(
  mut dices: Query<(&Transform, &Dice)>,
  mut event_writer: EventWriter<RollResultEvent>
) {
  let mut results = Vec::new();
  for (transform, dice) in &mut dices {
    let face_id = get_face_id(transform.rotation);
    if dice.id() == (DiceID { team_id: 0, dice_id: 0 }) {
      info!("Dice result: {}", face_id+1);
    }
    results.push((dice.id(), face_id));
  }
  event_writer.send(RollResultEvent(results));
}

fn get_face_id(rotation: Quat) -> usize {
  let mut face_id = 0;
  let mut max_dot = -1.0;
  let face_normals = [
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, -1.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
  ];
  for i in 0..6 {
    let dot = rotation.mul_vec3(face_normals[i]).dot(Vec3::new(0.0, 0.0, 1.0));
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
