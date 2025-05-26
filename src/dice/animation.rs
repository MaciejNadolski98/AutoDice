use bevy::prelude::*;
use bevy_inspector_egui::inspector_options::Target;

use crate::constants::{ANGULAR_SPEED, DICE_SIZE, HEIGHT, LINEAR_SPEED};
use crate::states::GameState;

use super::dice_instance::DiceEntityMap;
use super::Dice;
use super::events::{MoveDice, MoveDiceToMiddle, MoveDiceToRow, MovementFinished, OrientDice, ShakeDice};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, handle_move_events.run_if(in_state(GameState::Battle)))
      .add_systems(Update, move_entities.run_if(in_state(GameState::Battle)));
  }
}

#[derive(Component)]
pub struct TransformTo {
  pub target: Transform,
  pub linear_speed: f32,
  pub angular_speed: f32,
}

fn handle_move_events(
  mut commands: Commands,
  mut dices: Query<(Entity, &Transform, &Dice)>,
  mut move_dice_reader: EventReader<MoveDice>,
  mut move_dice_to_middle_reader: EventReader<MoveDiceToMiddle>,
  mut move_dice_to_row_reader: EventReader<MoveDiceToRow>,
  mut shake_dice_reader: EventReader<ShakeDice>,
  mut orient_dice_reader: EventReader<OrientDice>,
  mut transform_to_query: Query<&mut TransformTo>,
  entity_map: Res<DiceEntityMap>,
) {
  let mut add_animation = |dice_entity: Entity, function: &dyn Fn(&TransformTo) -> TransformTo| {
    if let Ok(mut transform_to) = transform_to_query.get_mut(dice_entity) {
      *transform_to = function(&*transform_to);
    } else {
      let (_, transform, _) = dices.get(dice_entity).unwrap();
      commands.entity(dice_entity).insert(function(&TransformTo {
        target: *transform,
        linear_speed: LINEAR_SPEED,
        angular_speed: ANGULAR_SPEED,
      }));
    }
  };
  for move_dice in move_dice_reader.read() {
    if let Some(dice_entity) = entity_map.0.get(&move_dice.dice_id) {
      add_animation(*dice_entity, &|transform_to: &TransformTo| {
        TransformTo {
          target: Transform::from_translation(transform_to.target.translation)
            .with_rotation(transform_to.target.rotation),
          linear_speed: LINEAR_SPEED,
          angular_speed: transform_to.angular_speed,
        }
      });
    }
  }

  for move_dice_to_middle in move_dice_to_middle_reader.read() {
    if let Some(dice_entity) = entity_map.0.get(&move_dice_to_middle.dice_id) {
      add_animation(*dice_entity, &|transform_to: &TransformTo| {
        let y = if move_dice_to_middle.dice_id.team_id == 0 { HEIGHT / 4.0 } else { -HEIGHT / 4.0 };
        TransformTo {
          target: Transform::from_translation(Vec3::new(0.0, y, DICE_SIZE / 2.0))
            .with_rotation(transform_to.target.rotation),
          linear_speed: LINEAR_SPEED,
          angular_speed: transform_to.angular_speed,
        }
      });
    }
  }

  for move_dice_to_row in move_dice_to_row_reader.read() {
    if let Some(dice_entity) = entity_map.0.get(&move_dice_to_row.dice_id) {
      add_animation(*dice_entity, &|transform_to: &TransformTo| {
        let y = if move_dice_to_row.dice_id.team_id == 0 { HEIGHT * 2.0 / 5.0 } else { -HEIGHT * 2.0 / 5.0 };
        let x = 0.0; // TODO: Calculate x based on the index within row
        TransformTo {
          target: Transform::from_translation(Vec3::new(x, y, DICE_SIZE / 2.0))
            .with_rotation(transform_to.target.rotation),
          linear_speed: LINEAR_SPEED,
          angular_speed: transform_to.angular_speed,
        }
      });
    }
  }

  for shake_dice in shake_dice_reader.read() {
    // TODO: Implement shake logic
  }

  for orient_dice in orient_dice_reader.read() {
    if let Some(dice_entity) = entity_map.0.get(&orient_dice.dice_id) {
      add_animation(*dice_entity, &|transform_to: &TransformTo| {
        let (rotation_x, rotation_y, _) = transform_to.target.rotation.to_euler(EulerRot::XYZ);
        let target_transform = Transform::from_translation(transform_to.target.translation)
          .with_rotation(Quat::from_euler(EulerRot::XYZ, rotation_x, rotation_y, 0.0));
        TransformTo {
          target: target_transform,
          linear_speed: LINEAR_SPEED,
          angular_speed: ANGULAR_SPEED,
        }
      });
    }
  }
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
    let (axis, angle) = rhs_angular_difference.to_axis_angle();
    if angle < angular_displacement {
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
