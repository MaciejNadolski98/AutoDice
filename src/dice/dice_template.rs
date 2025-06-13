use bevy::prelude::*;

use crate::dice::{face::{spawn_dice_faces, Face, FaceCollection}, GridableFaceCollection};

use super::action::Action;

pub struct DiceTemplatePlugin;

impl Plugin for DiceTemplatePlugin {
  fn build(&self, _app: &mut App) {
  }
}

#[derive(Component, Clone)]
#[component(on_add = spawn_dice_faces::<DiceTemplate>)]
pub struct DiceTemplate {
  pub hp: u32,
  pub faces: [Face; 6],
}

impl FaceCollection for DiceTemplate {
  fn faces(&self) -> Vec<Face> {
    self.faces.clone().into()
  }
}

impl GridableFaceCollection for DiceTemplate {
  fn gridded_faces(&self) -> Vec<(i16, i16, Face)> {
    [(2, 1), (1, 2), (3, 2), (2, 2), (2, 3), (2, 4)]
      .into_iter().zip(self.faces.clone().into_iter())
      .map(|((x, y), face)| (x, y, face))
      .collect()
  }
}

impl DiceTemplate {
  pub fn generate(images: &mut Assets<Image>) -> Self {
    Self {
      hp: 10,
      faces: [
        Face::new(Action::Attack, 2, images),
        Face::new(Action::Attack, 1, images),
        Face::new(Action::Attack, 1, images),
        Face::new(Action::Defend, 1, images),
        Face::new(Action::Regenerate, 1, images),
        Face::new(Action::Fire, 2, images),
      ]
    }
  }
}
