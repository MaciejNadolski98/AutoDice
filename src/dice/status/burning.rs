use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use bevy_defer::AccessError;

use crate::{
  dice::{
    action::interaction::dice::damage,
    DiceID
  },
  battle::StartRound
};

use super::Status;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Burning {
  pub intensity: u32,
}

impl Status for Burning {
    type TriggerEvent = StartRound;

  async fn resolve_status(&self, dice_id: DiceID, _event: Arc<Mutex<Self::TriggerEvent>>) -> Result<(), AccessError> {
    damage(dice_id, self.intensity).await?;
    Ok(())
  }
  
  fn update(&mut self) -> bool {
    self.intensity -= 1;
    self.intensity == 0
  }

  fn combine(self, other: Self) -> Self {
    Self {
      intensity: self.intensity + other.intensity,
    }
  }
}
