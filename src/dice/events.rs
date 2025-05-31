use bevy::prelude::*;
use bevy_defer::AccessError;

use super::DiceID;

#[derive(Clone, Copy, Default)]
pub struct FaceDescription {
  pub action_type: ActionType,
  #[allow(dead_code)]
  pub pips_count: u32,
}

impl FaceDescription {
  pub async fn resolve(self) -> Result<(), AccessError> {
    // TODO: Implement the logic to resolve the action based on the face description.
    Ok(())
  }
}

#[derive(Clone, Copy, Default)]
pub enum ActionType {
  #[default]
  Invalid,
  Attack,
  Heal,
  Defend,
  Fire,
  Digit1,
  Digit2,
  Digit3,
  Digit4,
  Digit5,
  Digit6,
}

pub struct DiceEventsPlugin;

impl Plugin for DiceEventsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<ChangeDiceFace>()
      .add_event::<SpawnDices>();
  }
}


#[derive(Event)]
pub struct ChangeDiceFace {
  pub dice_id: DiceID,
  pub face_id: usize,
  pub face: FaceDescription,
}

#[derive(Event)]
pub struct SpawnDices;
