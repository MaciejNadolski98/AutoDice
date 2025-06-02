use bevy::prelude::*;

use super::{debug_control::DebugControlPlugin, scene::ScenePlugin, sequence::SequencePlugin, floating_text::FloatingTextPlugin};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins((
        SequencePlugin,
        ScenePlugin,
        DebugControlPlugin,
        FloatingTextPlugin,
      ));
  }
}
