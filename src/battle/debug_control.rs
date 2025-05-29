use bevy::prelude::*;

use crate::{camera::BattleCamera, constants::DICE_SIZE, dice::{DiceID, MoveDiceToRow, OrientDice, SpinDice, TossDices}, states::GameState};

pub struct DebugControlPlugin;

impl Plugin for DebugControlPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, debug_control.run_if(in_state(GameState::Battle)));
  }
}

// For debug
fn debug_control(
  keys: Res<ButtonInput<KeyCode>>,
  mut toss_dices: EventWriter<TossDices>,
  mut battle_camera: Query<&mut Transform, With<BattleCamera>>,
  mut move_dices: EventWriter<MoveDiceToRow>,
  mut orient_dices: EventWriter<OrientDice>,
  mut spin_dice_writer: EventWriter<SpinDice>,
) {
  if keys.just_pressed(KeyCode::KeyQ) {
    toss_dices.send(TossDices {});
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
    spin_dice_writer.send(SpinDice {
      dice_id: DiceID { team_id: 0, dice_id: digit - 1 },
    });
  }

  if keys.just_pressed(KeyCode::KeyC) {
    for dice_id in 0..5 {
      for team_id in 0..2 {
        move_dices.send(MoveDiceToRow {
          dice_id: DiceID { team_id: team_id, dice_id: dice_id },
        });
        orient_dices.send(OrientDice {
          dice_id: DiceID { team_id: team_id, dice_id: dice_id },
        });
      }
    }
  }
}
