use std::f32::EPSILON;

use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_player)
      .add_systems(Update, character_movement)
      .register_type::<Player>();
  }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
  #[inspector(min = 0.0f32)]
  pub speed: f32,
}

fn spawn_player(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  let texture = asset_server.load("hero.png");

  commands.spawn((
      SpriteBundle {
          texture,
          ..default()
      },
      Player { speed: 100.0 },
      Name::new("Player"),
  ));
}

fn character_movement(
  mut characters: Query<(&mut Transform, &Player)>,
  input: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
) {
  for (mut transform, player) in &mut characters {
    let mut direction = Vec3 { ..default() };
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if direction.length() < EPSILON {
        continue;
    }
    
    let movement_amount = player.speed * time.delta_seconds();
    let movement_vector = movement_amount * direction.normalize();
    
    transform.translation += movement_vector;
  }
}
