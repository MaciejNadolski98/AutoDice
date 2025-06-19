use bevy::prelude::*;

use crate::states::GameState;

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
  mut time: ResMut<Time<Virtual>>,
) {
  if keys.just_pressed(KeyCode::Space) {
    if time.is_paused() {
      time.unpause();
    } else {
      time.pause();
    }
  }

  let key = if keys.just_pressed(KeyCode::Digit1) {
    Some(1.0)
  } else if keys.just_pressed(KeyCode::Digit2) {
    Some(2.0)
  } else if keys.just_pressed(KeyCode::Digit3) {
    Some(3.0)
  } else if keys.just_pressed(KeyCode::Digit4) {
    Some(4.0)
  } else if keys.just_pressed(KeyCode::Digit5) {
    Some(5.0)
  } else {
    None
  };

  if let Some(speedup) = key {
    let scale = 2.0f32.powf(speedup - 1.0);
    time.set_relative_speed(scale);
  }
}
