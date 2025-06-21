use bevy::prelude::*;
use bevy_defer::{AccessError, AsyncWorld};
use crate::{dice::{background::FaceBackground, FacePrototype}, utils::*};

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
  Empty,
  Attack,
  Defend,
  Regenerate,
  Fire,
  Fiery,
}

impl From<Action> for &'static str {
  fn from(action: Action) -> Self {
    match action {
      Action::Empty => "actions/empty.png",
      Action::Attack => "actions/axe.png",
      Action::Defend => "actions/shield.png",
      Action::Regenerate => "actions/heart.png",
      Action::Fire => "actions/fire.png",
      Action::Fiery => "actions/potion_red.png",
    }
  }
}

#[derive(Event, Clone, Copy, Debug)]
pub struct GetPips {
  pub dice_id: DiceID,
  pub pips: u32,
}

#[derive(Clone, Copy)]
pub struct ResolutionContext {
  pub face: FacePrototype,
  pub dice_id: DiceID,
}

pub async fn resolve(
  context: ResolutionContext,
) -> Result<(), AccessError> {
  let ResolutionContext { face, dice_id } = context;
  let FacePrototype { action, pips, background } = face;
  let repeat = if background == FaceBackground::Double { 2 } else { 1 };
  for _ in 0..repeat {
    match action {
      Action::Empty => Ok(()),
      Action::Attack => attack(get_pips(dice_id, pips.unwrap()).await?, context).await,
      Action::Defend => double(context).await,
      Action::Regenerate => regenerate(get_pips(dice_id, pips.unwrap()).await?, context).await,
      Action::Fire => fire(get_pips(dice_id, pips.unwrap()).await?, context).await,
      Action::Fiery => Ok(()),
    }?
  }
  Ok(())
}

async fn get_pips(
  dice_id: DiceID,
  pips: u32
) -> Result<u32, AccessError> {
  let get_pips = GetPips::wrap(GetPips { dice_id, pips: pips });
  AsyncWorld.trigger_event(get_pips.clone()).await?;
  Ok(get_pips.get().pips)
}
