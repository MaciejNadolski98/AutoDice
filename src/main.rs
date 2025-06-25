use bevy::prelude::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy::window::WindowResolution;
use bevy_defer::AsyncPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use constants::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH};

mod states;
mod menu;
mod battle;
mod manage;
mod camera;
mod dice;
mod constants;
mod utils;
mod loading_screen;

use crate::camera::plugin::CameraPlugin;
use crate::battle::plugin::BattlePlugin;
use crate::dice::plugin::DicePlugin;
use crate::menu::plugin::MenuPlugin;
use crate::manage::plugin::ManagePlugin;
use crate::loading_screen::LoadingScreenPlugin;
use crate::utils::tooltip::TooltipPlugin;
use crate::states::GameState;

fn main() {
  let mut app = App::new();
  app
    .add_plugins(AsyncPlugin::default_settings())
    .add_plugins(
      DefaultPlugins
      .set(WindowPlugin {
        primary_window: Some(Window {
          title: "Auto Dice".to_string(),
          resolution: WindowResolution::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT),
          ..Default::default()
        }),
        ..default()
      })
      .set(ImagePlugin::default_nearest())
    )
    .init_state::<GameState>()
    .add_plugins((MenuPlugin, ManagePlugin, BattlePlugin, CameraPlugin, DicePlugin, LoadingScreenPlugin, TooltipPlugin))
    .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true });
  
  if cfg!(debug_assertions) {
    app
      .add_plugins(
          WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
      );
  }
  app.run();
}
