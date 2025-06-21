use bevy::prelude::*;

use crate::dice::status::RegisterStatus;

use super::{Burning, Double, Regeneration};

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
  fn build(&self, app: &mut App) {
    app
      .register::<Burning>()
      .register::<Double>()
      .register::<Regeneration>();
  }
}
