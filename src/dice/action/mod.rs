use bevy::prelude::*;
use bevy_defer::{AccessError, AsyncWorld};
use crate::utils::*;

use super::DiceID;

mod attack;
mod fire;
mod double;
mod regenerate;

mod helpers;
pub mod interaction;

use attack::attack;
use fire::fire;
use double::double;
use regenerate::regenerate;

pub struct DiceActionPlugin;

impl Plugin for DiceActionPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event_and_listen::<GetPips>();
  }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
  #[default]
  Invalid,
  Attack,
  Defend,
  Regenerate,
  Fire,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct GetPips {
  pub dice_id: DiceID,
  pub pips: u32,
}

impl Action {
  pub async fn resolve(
    self,
    mut pips_count: u32,
    dice_id: DiceID,
  ) -> Result<(), AccessError> {
    let get_pips = GetPips::new(GetPips { dice_id, pips: pips_count });
    AsyncWorld.trigger_event(get_pips.clone()).await?;
    pips_count = get_pips.get().pips;
    match self {
      Action::Attack => attack(pips_count, dice_id).await,
      Action::Defend => double(dice_id).await,
      Action::Regenerate => regenerate(pips_count, dice_id).await,
      Action::Fire => fire(pips_count, dice_id).await,
      Action::Invalid => Err(AccessError::Custom("Invalid action")),
    }
  }
}

async fn placeholder() -> Result<(), AccessError> {
  Ok(())
}
