use bevy::prelude::*;

use super::{Burning, Double, Regeneration, RegisterRegistrable};

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
  fn build(&self, app: &mut App) {
    app
      .register::<Burning>()
      .register::<Double>()
      .register::<Regeneration>();
  }
}
