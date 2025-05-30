use bevy::prelude::*;

use crate::{camera::BattleCamera, constants::DICE_SIZE, states::GameState};

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
  mut battle_camera: Query<&mut Transform, With<BattleCamera>>,
) {
  if keys.pressed(KeyCode::ArrowUp) {
    battle_camera.single_mut().unwrap().rotate_local_x(0.1);
  }

  if keys.pressed(KeyCode::ArrowDown) {
    battle_camera.single_mut().unwrap().rotate_local_x(-0.1);
  }

  if keys.pressed(KeyCode::ArrowRight) {
    battle_camera.single_mut().unwrap().rotate_local_y(-0.1);
  }

  if keys.pressed(KeyCode::ArrowLeft) {
    battle_camera.single_mut().unwrap().rotate_local_y(0.1);
  }

  if keys.pressed(KeyCode::KeyW) {
    let mut transform = battle_camera.single_mut().unwrap();
    let rotation = transform.rotation;
    let translation = transform.translation;
    let up = rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0));
    transform.translation = translation.move_towards(translation + up, 0.1 * DICE_SIZE);
  }

  if keys.pressed(KeyCode::KeyS) {
    let mut transform = battle_camera.single_mut().unwrap();
    let rotation = transform.rotation;
    let translation = transform.translation;
    let up = rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0));
    transform.translation = translation.move_towards(translation + up, -0.1 * DICE_SIZE);
  }

  if keys.pressed(KeyCode::KeyD) {
    let mut transform = battle_camera.single_mut().unwrap();
    let rotation = transform.rotation;
    let translation = transform.translation;
    let right = rotation.mul_vec3(Vec3::new(1.0, 0.0, 0.0));
    transform.translation = translation.move_towards(translation + right, 0.1 * DICE_SIZE);
  }

  if keys.pressed(KeyCode::KeyA) {
    let mut transform = battle_camera.single_mut().unwrap();
    let rotation = transform.rotation;
    let translation = transform.translation;
    let right = rotation.mul_vec3(Vec3::new(1.0, 0.0, 0.0));
    transform.translation = translation.move_towards(translation + right, -0.1 * DICE_SIZE);
  }

  if keys.pressed(KeyCode::KeyX) {
    let mut transform = battle_camera.single_mut().unwrap();
    let rotation = transform.rotation;
    let translation = transform.translation;
    let forward = rotation.mul_vec3(Vec3::new(0.0, 0.0, 1.0));
    transform.translation = translation.move_towards(translation + forward, 0.1 * DICE_SIZE);
  }

  if keys.pressed(KeyCode::KeyZ) {
    let mut transform = battle_camera.single_mut().unwrap();
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
}
