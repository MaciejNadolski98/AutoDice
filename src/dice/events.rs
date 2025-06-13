use bevy::prelude::*;

use crate::utils::*;

use super::DiceID;

pub struct DiceEventsPlugin;

impl Plugin for DiceEventsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event_and_listen::<SpawnDices>()
      .add_event_and_listen::<DiceDied>();
  }
}

#[derive(Event)]
pub struct SpawnDices;

#[derive(Event, Clone, Copy, Debug)]
pub struct DiceDied {
  pub dice_id: DiceID,
}
