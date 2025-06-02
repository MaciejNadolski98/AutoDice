use bevy::prelude::*;

use super::{
  animation::AnimationPlugin,
  dice_instance::DiceInstancePlugin,
  dice_render::DiceRenderPlugin,
  dice_template::DiceTemplatePlugin,
  events::DiceEventsPlugin,
  health_bar::HealthBarPlugin,
  roll::RollPlugin,
};

pub struct DicePlugin;

impl Plugin for DicePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins((
        DiceEventsPlugin,
        AnimationPlugin,
        DiceInstancePlugin,
        DiceRenderPlugin,
        DiceTemplatePlugin,
        HealthBarPlugin,
        RollPlugin,
      ));
  }
}
