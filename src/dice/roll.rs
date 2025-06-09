use avian3d::prelude::*;
use bevy::prelude::*;
use rand_distr::{Distribution, Normal};

use crate::constants::{ANGULAR_VELOCITY_EPSILON, DICE_SIZE, FACE_NORMALS, HEIGHT, LINEAR_VELOCITY_EPSILON, WIDTH};

use super::{dice_instance::{DiceEntityMap, RowPositionMappings}, events::{DicesStopped, RollResult, RowPositionChanged}, Dice, DiceID, TossDices};

pub struct RollPlugin;

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq)]
pub struct DicesRolling(pub bool);

impl Plugin for RollPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<DicesRolling>()
      .add_systems(Update, roll_dices.run_if(on_event::<TossDices>))
      .add_systems(Update, check_if_dices_stopped.run_if(resource_equals::<DicesRolling>(DicesRolling(true))))
      .add_systems(Update, (check_roll_results, compute_row_positions).run_if(on_event::<DicesStopped>))
      .add_systems(Update, update_row_positions);
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
  mut event_writer: EventWriter<DicesStopped>
) {
  for (linear_velocity, angular_velocity) in &dices {
    if linear_velocity.0.length() > LINEAR_VELOCITY_EPSILON || angular_velocity.0.length() > ANGULAR_VELOCITY_EPSILON {
      return;
    }
  }
  dices_rolling.0 = false;
  event_writer.send(DicesStopped);
}

fn check_roll_results(
  mut dices: Query<(&Transform, &Dice)>,
  mut event_writer: EventWriter<RollResult>
) {
  let mut results = Vec::new();
  for (transform, dice) in &mut dices {
    let face_id = get_face_id(transform.rotation);
    results.push((dice.id(), face_id));
  }
  event_writer.send(RollResult(results));
}

fn compute_row_positions(
  mut dices: Query<(&Transform, &mut Dice)>,
  mut row_position_changed: EventWriter<RowPositionChanged>,
  dice_mapping: Res<DiceEntityMap>,
) {
  let mut team_1_dices = Vec::new();
  let mut team_2_dices = Vec::new();

  for (transform, dice) in &mut dices {
    if dice.id().team_id == 0 {
      team_1_dices.push((transform.translation.x, dice.id()));
    } else {
      team_2_dices.push((transform.translation.x, dice.id()));
    }
  }

  team_1_dices.sort_by(|(a, _), (b, _)| a.partial_cmp(&b).unwrap());
  team_2_dices.sort_by(|(a, _), (b, _)| a.partial_cmp(&b).unwrap());

  for (i, (_, dice_id)) in team_1_dices.iter().enumerate() {
    let entity = dice_mapping.0.get(dice_id).unwrap();
    let mut dice = dices.get_mut(*entity).unwrap();
    row_position_changed.send(dice.1.set_row_position(i));
  }

  for (i, (_, dice_id)) in team_2_dices.iter().enumerate() {
    let entity = dice_mapping.0.get(dice_id).unwrap();
    let mut dice = dices.get_mut(*entity).unwrap();
    row_position_changed.send(dice.1.set_row_position(i));
  }
}

fn update_row_positions(
  mut row_position_changed: EventReader<RowPositionChanged>,
  mut row_mappings: ResMut<RowPositionMappings>,
) {
  for event in row_position_changed.read() {
    if event.dice_id.team_id == 0 {
      row_mappings.team1.insert(event.position, event.dice_id);
    } else {
      row_mappings.team2.insert(event.position, event.dice_id);
    }
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
