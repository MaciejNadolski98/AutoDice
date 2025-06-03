use bevy::prelude::*;
use bevy_defer::AccessError;

use crate::{
  dice::{
    action::interaction::dice::heal,
    DiceID
  },
  battle::StartRound
};

use super::Status;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Regeneration {
  pub heal_amount: u32,
  pub duration_left: u32,
}

impl Status for Regeneration {
    type TriggerEvent = StartRound;

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
}
