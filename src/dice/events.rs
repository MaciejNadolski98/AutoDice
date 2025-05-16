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
}

pub struct DiceEventsPlugin;

impl Plugin for DiceEventsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<DiceFaceChangedEvent>()
      .add_event::<TossDicesEvent>();
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
