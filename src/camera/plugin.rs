
use bevy::prelude::*;

use super::battle_camera::BattleCameraPlugin;
use super::menu_camera::MenuCameraPlugin;
use super::tooltip_camera::TooltipCameraPlugin;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins((BattleCameraPlugin, MenuCameraPlugin, TooltipCameraPlugin));
  }
}
