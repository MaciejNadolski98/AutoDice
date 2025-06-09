use bevy::prelude::*;
use bevy_defer::AppReactorExtension;

use super::{dice_template::Face, DiceID};

pub struct DiceEventsPlugin;

impl Plugin for DiceEventsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<ChangeDiceFace>()
      .add_event::<SpawnDices>()
      .add_event::<DiceDied>()
      .react_to_event::<DiceDied>();
  }
}


#[derive(Event)]
pub struct ChangeDiceFace {
  pub dice_id: DiceID,
  pub face_id: usize,
  pub face: Face,
}

#[derive(Event)]
pub struct SpawnDices;

#[derive(Event, Clone, Copy, Debug)]
pub struct DiceDied {
  pub dice_id: DiceID,
}
