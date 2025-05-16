use bevy::prelude::*;

use super::{ActionType, FaceDescription};

pub struct DiceTemplatePlugin;

impl Plugin for DiceTemplatePlugin {
  fn build(&self, app: &mut App) {
    app;
  }
}

#[derive(Clone)]
pub struct DiceTemplate {
  pub hp: u32,
  pub faces: [FaceDescription; 6],
}

impl DiceTemplate {
  pub fn generate() -> Self {
    Self {
      hp: 10,
      faces: [
        FaceDescription { action_type: ActionType::Attack, pips_count: 2 },
        FaceDescription { action_type: ActionType::Attack, pips_count: 1 },
        FaceDescription { action_type: ActionType::Attack, pips_count: 1 },
        FaceDescription { action_type: ActionType::Defend, pips_count: 1 },
        FaceDescription { action_type: ActionType::Heal, pips_count: 1 },
        FaceDescription { action_type: ActionType::Fire, pips_count: 0 },
      ],
    }
  }
}
