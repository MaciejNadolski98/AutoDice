use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::dice::{face::Face, Gridable};

use super::action::Action;

pub struct DiceTemplatePlugin;

impl Plugin for DiceTemplatePlugin {
  fn build(&self, _app: &mut App) {
  }
}

#[derive(Component, Clone)]
pub struct DiceTemplate {
  pub hp: u32,
}

impl Gridable for DiceTemplate {
  fn grid(&self) -> Vec<(i16, i16)> {
    vec![(2, 1), (1, 2), (3, 2), (2, 2), (2, 3), (2, 4)]
  }
}

impl DiceTemplate {
  pub fn spawn(mut images: &mut Assets<Image>, commands: &mut RelatedSpawnerCommands<ChildOf>) {
    let template = Self { hp: 10 };
    commands
      .spawn(template)
      .with_children(|commands| {
        [
          Face::new(Action::Attack, 2, &mut images),
          Face::new(Action::Attack, 2, &mut images),
          Face::new(Action::Attack, 2, &mut images),
          Face::new(Action::Attack, 2, &mut images),
          Face::new(Action::Attack, 2, &mut images),
          Face::new(Action::Attack, 2, &mut images),
        ].map(|face|{
          commands.spawn(face);
        });
      });
  }
}
