use bevy::{prelude::*, input::common_conditions::input_toggle_active};
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod pigs;
mod player;
mod ui;

use pigs::PigPlugin;
use player::{PlayerPlugin, Player};
use ui::GameUI;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(PigPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GameUI)
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin { min_width: 254.0, min_height: 144.0 };

    commands.spawn(camera);
}

#[derive(Resource)]
pub struct Money(pub f32);
