use bevy::prelude::*;
use bevy_defer::{AccessError, AsyncWorld};

use crate::{battle::StartGame, dice::{action::interaction::dice::apply_status, status::Burning, synergy::Synergy, Action, Dice, FacePrototype}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fiery {
  intensity: u32,
  team_id: usize,
}

impl Synergy for Fiery {
  type TriggerEvent = StartGame;

  const SYNERGY_COLOR: Color = Color::linear_rgb(1.0, 0.5, 0.5);

  fn new(intensity: u32, team_id: usize) -> Option<Self> {
    if intensity > 0 {
      Some(Self { intensity, team_id })
    } else {
      None
    }
  }

  fn name() -> &'static str {
    "Fiery"
  }

  fn description() -> &'static str {
    "Applies Burning to all opponents at the start of battle
    1 -> 1 Burning
    3 -> 2 Burning
    5 -> 3 Burning
    "
  }

  async fn resolve(&self, _event: Self::TriggerEvent) -> Result<(), AccessError> {
    let mut dices = Vec::new();

    AsyncWorld
      .query::<&Dice>()
      .for_each(|dice| {
        if dice.id().team_id != self.team_id {
          dices.push(dice.id());
        }
      });

    for dice_id in dices {
      apply_status(dice_id, Burning { intensity: self.level() }).await?;
    }
    Ok(())
  }

  fn intensity(&self) -> u32 {
    self.intensity
  }

  fn break_points(&self) -> &[u32] {
    &[1, 3, 5]
  }

  fn read_face(face: FacePrototype) -> u32 {
    if face.action == Action::Fiery { 1 } else { 0 }
  }
} 
