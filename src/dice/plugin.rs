use bevy::prelude::*;

use super::{
  animation::AnimationPlugin,
  dice_instance::DiceInstancePlugin,
  dice_render::DiceRenderPlugin,
  dice_template::DiceTemplatePlugin,
  events::DiceEventsPlugin,
  dice_info_bar::DiceInfoBarPlugin,
  roll::RollPlugin,
  action::DiceActionPlugin,
  status::StatusPlugin,
  face::FacePlugin,
  synergy::SynergyPlugin,
};

pub struct DicePlugin;

impl Plugin for DicePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(MeshPickingPlugin)
      .add_plugins((
        DiceEventsPlugin,
        AnimationPlugin,
        DiceInstancePlugin,
        DiceRenderPlugin,
        DiceTemplatePlugin,
        DiceInfoBarPlugin,
        RollPlugin,
        DiceActionPlugin,
        StatusPlugin,
        FacePlugin,
        SynergyPlugin,
      ));
  }
}
