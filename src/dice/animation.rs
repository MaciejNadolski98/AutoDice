use avian3d::math::PI;
use avian3d::prelude::RigidBodyDisabled;
use bevy::prelude::*;

use crate::constants::{ANGULAR_SPEED, DICE_SIZE, FACE_NORMALS, HEIGHT, LINEAR_SPEED, SPIN_ANGULAR_SPEED, SPIN_DURATION};
use crate::states::GameState;

use super::dice_instance::DiceEntityMap;
use super::{Dice, TossDices};
use super::events::{DicesStopped, MoveDice, MoveDiceToMiddle, MoveDiceToRow, MovementFinished, OrientDice, SpinDice, SpinFinished};
use super::roll::get_face_id;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (
        handle_move_to_middle_events,
        handle_move_to_row_events,
        handle_move_events,
        handle_orient_dice_events,
        handle_spin_dice_events,
        move_entities,
        spin_dices,
      ).chain().run_if(in_state(GameState::Battle)))
      .add_systems(Update, add_physics.run_if(on_event::<TossDices>))
      .add_systems(Update, remove_physics.run_if(on_event::<DicesStopped>));
  }
}

#[derive(Component, Debug)]
pub struct TransformTo {
  pub target: Transform,
  pub linear_speed: f32,
  pub angular_speed: f32,
}

#[derive(Event)]
pub struct Spin {
  pub spin_axis: Vec3,
  pub angular_speed: f32,
  pub timer: Timer,
}

fn handle_move_events(
  mut commands: Commands,
  dices: Query<(Entity, &Transform, &Dice)>,
  transforms_to: Query<&TransformTo>,
  mut move_dice_reader: EventReader<MoveDice>,
  entity_map: Res<DiceEntityMap>,
) {
  for move_dice in move_dice_reader.read() {
    if let Some(dice_entity) = entity_map.0.get(&move_dice.dice_id) {
      let (_, transform, _) = dices.get(*dice_entity).unwrap();
      let transform_to = if let Ok(transform_to) = transforms_to.get(*dice_entity) {
        TransformTo {
          target: Transform::from_translation(move_dice.target_position)
            .with_rotation(transform_to.target.rotation),
          linear_speed: LINEAR_SPEED,
          angular_speed: transform_to.angular_speed,
        }
      } else {
        TransformTo {
          target: Transform::from_translation(move_dice.target_position)
            .with_rotation(transform.rotation),
          linear_speed: LINEAR_SPEED,
          angular_speed: ANGULAR_SPEED,
        }
      };
      commands.entity(*dice_entity).insert(transform_to);
    }
  }     
}

fn handle_move_to_middle_events(
  mut move_dice_to_middle_reader: EventReader<MoveDiceToMiddle>,
  mut move_dice_writer: EventWriter<MoveDice>,
) {
  for move_dice_to_middle in move_dice_to_middle_reader.read() {
    let target_y = if move_dice_to_middle.dice_id.team_id == 0 { HEIGHT / 5.0 } else { -HEIGHT / 5.0 };
    let target_position = Vec3::new(0.0, target_y, DICE_SIZE / 2.0);
    move_dice_writer.send(MoveDice {
      dice_id: move_dice_to_middle.dice_id,
      target_position,
    });
  }
}

fn handle_move_to_row_events(
  mut move_dice_to_row_reader: EventReader<MoveDiceToRow>,
  mut move_dice_writer: EventWriter<MoveDice>,
) {
  for move_dice_to_row in move_dice_to_row_reader.read() {
    let target_y = if move_dice_to_row.dice_id.team_id == 0 { HEIGHT * 2.0 / 5.0 } else { -HEIGHT * 2.0 / 5.0 };
    let target_x: f32 = 0.0; // TODO: Calculate x based on the index within row
    let target_position = Vec3::new(target_x, target_y, DICE_SIZE / 2.0);
    move_dice_writer.send(MoveDice {
      dice_id: move_dice_to_row.dice_id,
      target_position,
    });
  }
}

fn handle_orient_dice_events(
  mut commands: Commands,
  dices: Query<(Entity, &Transform, &Dice)>,
  transforms_to: Query<&TransformTo>,
  mut orient_dice_reader: EventReader<OrientDice>,
  entity_map: Res<DiceEntityMap>,
) {
  for orient_dice in orient_dice_reader.read() {
    if let Some(dice_entity) = entity_map.0.get(&orient_dice.dice_id) {
      let (_, transform, _) = dices.get(*dice_entity).unwrap();
      let target_rotation = compute_target_rotation(transform.rotation);
      let transform_to = if let Ok(transform_to) = transforms_to.get(*dice_entity) {
        TransformTo {
          target: Transform::from_translation(transform_to.target.translation)
            .with_rotation(target_rotation),
          linear_speed: transform_to.linear_speed,
          angular_speed: ANGULAR_SPEED,
        }
      } else {
        TransformTo {
          target: Transform::from_translation(transform.translation)
            .with_rotation(target_rotation),
          linear_speed: LINEAR_SPEED,
          angular_speed: ANGULAR_SPEED,
        }
      };
      commands.entity(*dice_entity).insert(transform_to);
    }
  }
}

fn handle_spin_dice_events(
  mut commands: Commands,
  dices: Query<&Transform>,
  mut spin_dice_reader: EventReader<SpinDice>,
  entity_map: Res<DiceEntityMap>,
) {
  for spin_dice in spin_dice_reader.read() {
    if let Some(dice_entity) = entity_map.0.get(&spin_dice.dice_id) {
      let transform = dices.get(*dice_entity).unwrap();
      commands.entity(*dice_entity).insert(Spin {
        spin_axis: FACE_NORMALS[get_face_id(transform.rotation)],
        angular_speed: SPIN_ANGULAR_SPEED,
        timer: Timer::from_seconds(SPIN_DURATION, TimerMode::Once),
      });
    }
  }
}

fn spin_dices(
  mut commands: Commands,
  mut spins: Query<(Entity, &Dice, &mut Transform, &mut Spin)>,
  time: Res<Time>,
  mut spin_finished_writer: EventWriter<SpinFinished>,
) {
  for (entity, dice, mut transform, mut spin) in &mut spins {
    let remaining_time = spin.timer.remaining_secs();
    let angle = f32::min(remaining_time, time.delta_secs()) * spin.angular_speed;
    spin.timer.tick(time.delta());
    transform.rotation = transform.rotation * Quat::from_axis_angle(spin.spin_axis, angle);
    if spin.timer.just_finished() {
      commands.entity(entity).remove::<Spin>();
      spin_finished_writer.send(SpinFinished { dice_id: dice.id() });
    }
  }
}

fn compute_target_rotation(current_rotation: Quat) -> Quat {
  let face_id = get_face_id(current_rotation);
  Quat::from_rotation_arc(FACE_NORMALS[face_id], Vec3::Z)
}

fn move_entities(
  mut commands: Commands,
  mut entities: Query<(Entity, &mut Transform, &TransformTo)>,
  time: Res<Time>,
  mut movement_finished: EventWriter<MovementFinished>,
) {
  let delta = time.delta_secs();
  let total_entities_to_move = entities.iter().count();
  let mut finished_movements = 0;
  for (entity, mut transform, transform_to) in &mut entities {
    let target = transform_to.target;
    let linear_displacement = transform_to.linear_speed * delta;
    let angular_displacement = transform_to.angular_speed * delta;
    let mut translation_completed = false;
    let mut rotation_completed = false;

    let linear_difference = target.translation - transform.translation;
    if linear_difference.length() < linear_displacement {
      transform.translation = target.translation;
      translation_completed = true;
    } else {
      transform.translation += linear_difference.normalize() * linear_displacement;
    }

    let rhs_angular_difference = transform.rotation.inverse() * target.rotation;
    let (mut axis, mut angle) = rhs_angular_difference.to_axis_angle();
    if angle > PI {
      // Normalize angle to be within [0, PI]
      angle = angle - 2.0 * PI;
      axis = -axis; // Reverse the axis if we normalize the angle
    }
    if angle.abs() < angular_displacement {
      transform.rotation = target.rotation;
      rotation_completed = true;
    } else {
      let rotation = Quat::from_axis_angle(axis, angular_displacement);
      transform.rotation = transform.rotation * rotation;
    }

    if translation_completed && rotation_completed {
      finished_movements += 1;
      commands.entity(entity).remove::<TransformTo>();
    }
  }

  if finished_movements == total_entities_to_move {
    movement_finished.send(MovementFinished);
  }
}

fn remove_physics(
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
