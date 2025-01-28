use bevy::prelude::*;

use super::dice_render::FaceDescription;

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
