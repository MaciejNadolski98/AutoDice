use bevy::prelude::*;
use bevy_defer::AccessError;

use super::{action::Action, DiceID};

pub struct DiceTemplatePlugin;

impl Plugin for DiceTemplatePlugin {
  fn build(&self, _app: &mut App) {
  }
}

#[derive(Clone)]
pub struct DiceTemplate {
  pub hp: u32,
  pub faces: [Face; 6],
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Face {
  pub action: Action,
  pub pips_count: u32,
}

impl Face {
  pub async fn resolve(self, dice_id: DiceID) -> Result<(), AccessError> {
    self.action.resolve(self.pips_count, dice_id).await
  }
}

impl DiceTemplate {
  pub fn generate() -> Self {
    Self {
      hp: 10,
      faces: [
        Face { action: Action::Attack, pips_count: 2 },
        Face { action: Action::Attack, pips_count: 1 },
        Face { action: Action::Attack, pips_count: 1 },
        Face { action: Action::Defend, pips_count: 1 },
        Face { action: Action::Regenerate, pips_count: 1 },
        Face { action: Action::Fire, pips_count: 2 },
      ]
    }
  }
}
