use bevy::prelude::*;
use bevy_defer::AccessError;

use crate::{
  battle::StartRound, dice::{
    action::interaction::dice::heal,
    DiceID
  }, impl_status_component
};

use super::Status;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Regeneration {
  pub heal_amount: u32,
  pub duration_left: u32,
}

impl_status_component!(Regeneration);

impl Status for Regeneration {
  type TriggerEvent = StartRound;
  const STATUS_COLOR: Color = Color::linear_rgb(0.0, 1.0, 0.0);

  async fn resolve_status(&self, dice_id: DiceID, _event: Self::TriggerEvent) -> Result<(), AccessError> {
    heal(dice_id, self.heal_amount).await
  }

  fn update(&mut self) -> bool {
    self.duration_left -= 1;
    self.duration_left == 0
  }

  fn combine(self, other: Self) -> Self {
    Self { 
      heal_amount: self.heal_amount + other.heal_amount, 
      duration_left: self.duration_left.max(other.duration_left),
    }
  }

  fn intensity(&self) -> Option<u32> {
    Some(self.heal_amount)
  }
}
