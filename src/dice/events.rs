use bevy::prelude::*;

use super::DiceID;

#[derive(Clone, Copy, Default)]
pub struct FaceDescription {
  pub action_type: ActionType,
  pub pips_count: u32,
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
      .add_event::<TossDices>()
      .add_event::<SpawnDices>()
      .add_event::<DicesStopped>()
      .add_event::<RollResult>();
  }
}


#[derive(Event)]
pub struct ChangeDiceFace {
  pub dice_id: DiceID,
  pub face_id: usize,
  pub face: FaceDescription,
}

#[derive(Event)]
pub struct TossDices;

#[derive(Event)]
pub struct SpawnDices;

#[derive(Event)]
pub struct DicesStopped;

#[derive(Event)]
pub struct RollResult(pub Vec<(DiceID, usize)>);
