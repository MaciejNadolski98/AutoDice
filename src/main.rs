use bevy::prelude::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod states;
mod menu;
mod battle;
mod manage;
mod camera;
mod constants;

use crate::camera::plugin::CameraPlugin;
use crate::battle::plugin::BattlePlugin;
use crate::menu::plugin::MenuPlugin;
use crate::manage::plugin::ManagePlugin;
use crate::states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins((MenuPlugin, ManagePlugin, BattlePlugin, CameraPlugin))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .run();
}
