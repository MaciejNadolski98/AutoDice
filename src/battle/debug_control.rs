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
}
