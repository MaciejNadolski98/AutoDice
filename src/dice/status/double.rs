use bevy::prelude::*;
use bevy_defer::AccessError;

use crate::{dice::{action::GetPips, Dice, DiceID}};

use super::Status;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Double;

impl Status for Double {
  type TriggerEvent = GetPips;

  fn trigger_condition(&self, dice: &Dice, event: GetPips) -> bool {
    event.dice_id == dice.id()
  }

  async fn resolve_status(&self, _dice_id: DiceID, _event: GetPips) -> Result<(), AccessError> {
    Ok(())
  }

  async fn update_event(&self, _dice_id: DiceID, event: Self::TriggerEvent) -> Result<Self::TriggerEvent, AccessError> {
    Ok(GetPips { pips: event.pips * 2, ..event })
  }
  
  fn update(&mut self) -> bool {
    true
  }

  fn combine(self, _other: Self) -> Self {
    Self
  }
}
