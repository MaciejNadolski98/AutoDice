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
      .add_event::<DiceFaceChangedEvent>()
      .add_event::<TossDicesEvent>()
      .add_event::<DiceSpawnEvent>()
      .add_event::<DicesStoppedEvent>()
      .add_event::<RollResultEvent>();
  }
}


#[derive(Event)]
pub struct DiceFaceChangedEvent {
  pub dice_id: DiceID,
  pub face_id: usize,
  pub face: FaceDescription,
}

#[derive(Event)]
pub struct TossDicesEvent;

#[derive(Event)]
pub struct DiceSpawnEvent;

#[derive(Event)]
pub struct DicesStoppedEvent;

#[derive(Event)]
pub struct RollResultEvent(pub Vec<(DiceID, usize)>);
