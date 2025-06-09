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

use crate::camera::plugin::CameraPlugin;
use crate::battle::plugin::BattlePlugin;
use crate::dice::plugin::DicePlugin;
use crate::menu::plugin::MenuPlugin;
use crate::manage::plugin::ManagePlugin;
use crate::states::GameState;

fn main() {
    App::new()
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
        .add_plugins((MenuPlugin, ManagePlugin, BattlePlugin, CameraPlugin, DicePlugin))
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .run();
}
