use bevy::prelude::*;
use bevy_defer::{AccessError, AsyncWorld};

use crate::{
  battle::StartRound, dice::{
    action::interaction::dice::damage, DiceID
  }, impl_status_component
};

use super::Status;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Burning {
  pub intensity: u32,
}

impl_status_component!(Burning);

impl Status for Burning {
  type TriggerEvent = StartRound;
  const STATUS_COLOR: Color = Color::linear_rgb(1.0, 0.0, 0.0);

  fn description() -> &'static str {
    "Deals damage at the start of a turn and decreases intensity"
  }

  async fn resolve_status(&self, dice_id: DiceID, _event: Self::TriggerEvent) -> Result<(), AccessError> {
    damage(dice_id, self.intensity, Color::linear_rgb(1.0, 0.0, 0.0)).await?;
    AsyncWorld.sleep(0.5).await;
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

  fn intensity(&self) -> Option<u32> {
    Some(self.intensity)
  }
}
