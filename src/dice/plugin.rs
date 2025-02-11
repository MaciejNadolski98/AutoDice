use bevy::prelude::*;

use super::{
  dice_collector::DiceCollectorPlugin,
  dice_instance::DiceInstancePlugin,
  dice_render::DiceRenderPlugin,
  dice_template::DiceTemplatePlugin,
  events::DiceEventsPlugin
};

pub struct DicePlugin;

impl Plugin for DicePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins((
        DiceInstancePlugin,
        DiceRenderPlugin,
        DiceTemplatePlugin,
        DiceEventsPlugin,
        DiceCollectorPlugin
      ));
  }
}
