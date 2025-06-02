use bevy::prelude::*;

use crate::utils::*;

use super::{dice_template::Face, DiceID};

pub struct DiceEventsPlugin;

impl Plugin for DiceEventsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event_and_listen::<ChangeDiceFace>()
      .add_event_and_listen::<SpawnDices>()
      .add_event_and_listen::<DiceDied>()
      .add_event_and_listen::<StartRound>();
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

#[derive(Event, Clone, Copy, Debug)]
pub struct StartRound;
