use bevy::prelude::*;

use super::{ActionType, FaceDescription};

pub struct DiceTemplatePlugin;

impl Plugin for DiceTemplatePlugin {
  fn build(&self, _app: &mut App) {
  }
}

#[derive(Clone)]
pub struct DiceTemplate {
  pub hp: u32,
  pub faces: [FaceDescription; 6],
}

impl DiceTemplate {
  pub fn generate(numbered: bool) -> Self {
    Self {
      hp: 10,
      faces: if numbered {[
        FaceDescription { action_type: ActionType::Digit1, pips_count: 1 },
        FaceDescription { action_type: ActionType::Digit2, pips_count: 2 },
        FaceDescription { action_type: ActionType::Digit3, pips_count: 3 },
        FaceDescription { action_type: ActionType::Digit4, pips_count: 4 },
        FaceDescription { action_type: ActionType::Digit5, pips_count: 5 },
        FaceDescription { action_type: ActionType::Digit6, pips_count: 6 },
      ]} else {[
        FaceDescription { action_type: ActionType::Attack, pips_count: 2 },
        FaceDescription { action_type: ActionType::Attack, pips_count: 1 },
        FaceDescription { action_type: ActionType::Attack, pips_count: 1 },
        FaceDescription { action_type: ActionType::Defend, pips_count: 1 },
        FaceDescription { action_type: ActionType::Heal, pips_count: 1 },
        FaceDescription { action_type: ActionType::Fire, pips_count: 0 },
      ]},
    }
  }
}
