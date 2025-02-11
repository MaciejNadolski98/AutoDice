use bevy::prelude::*;

pub struct FaceDescription {
  pub action_type: ActionType,
  pub pips_count: u32,
}

pub enum ActionType {
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
    .add_event::<RespawnDicesEvent>();
  }
}


#[derive(Event)]
pub struct DiceFaceChangedEvent {
  pub team_id: u32,
  pub dice_id: u32,
  pub face_id: u32,
  pub face: FaceDescription,
}

#[derive(Event)]
pub struct RespawnDicesEvent;

#[derive(Event)]
pub struct DiceStationaryEvent;
