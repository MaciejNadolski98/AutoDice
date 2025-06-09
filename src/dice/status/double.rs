use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use bevy_defer::AccessError;

use crate::{dice::{action::GetPips, Dice, DiceID}, utils::*};

use super::Status;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Double;

impl Status for Double {
  type TriggerEvent = GetPips;

  fn trigger_condition(&self, dice: &Dice, event: Arc<Mutex<GetPips>>) -> bool {
    event.get().dice_id == dice.id()
  }

  async fn resolve_status(&self, _dice_id: DiceID, event: Arc<Mutex<GetPips>>) -> Result<(), AccessError> {
    event.mutate(|event| GetPips { pips: event.pips * 2, ..event });
    Ok(())
  }
  
  fn update(&mut self) -> bool {
    true
  }

  fn combine(self, _other: Self) -> Self {
    Self
  }
}
