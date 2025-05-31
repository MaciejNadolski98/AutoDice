use bevy_defer::AccessError;

use super::DiceID;

mod attack;

mod helpers;
mod interaction;

use attack::attack;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
  #[default]
  Invalid,
  Attack,
  Defend,
  Heal,
  Fire,
}

impl Action {
  pub async fn resolve(
    self,
    pips_count: u32,
    dice_id: DiceID,
  ) -> Result<(), AccessError> {
    match self {
      Action::Attack => attack(pips_count, dice_id).await,
      Action::Defend => placeholder().await,
      Action::Heal => placeholder().await,
      Action::Fire => placeholder().await,
      Action::Invalid => Err(AccessError::Custom("Invalid action")),
    }
  }
}

async fn placeholder() -> Result<(), AccessError> {
  Ok(())
}
