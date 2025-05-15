use bevy::{prelude::*, render::camera::ScalingMode};
use crate::states::GameState;

#[derive(Component)]
struct MenuCamera;

pub struct MenuCameraPlugin;

impl Plugin for MenuCameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Menu), spawn_menu_camera)
      .add_systems(OnExit(GameState::Menu), despawn_menu_camera)
      .add_systems(OnEnter(GameState::Manage), spawn_menu_camera)
      .add_systems(OnExit(GameState::Manage), despawn_menu_camera);
  }
}

fn spawn_menu_camera(
  mut commands: Commands,
) {
  commands.spawn((
    Name::new("Menu camera"),
    Camera2d,
    OrthographicProjection {
      scaling_mode: ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
      },
      ..OrthographicProjection::default_2d()
    },
    MenuCamera,
  ));
}

fn despawn_menu_camera(
  mut commands: Commands,
  camera_: Query<Entity, With<MenuCamera>>,
) {
  let camera = camera_.single();
  commands.entity(camera).despawn();
}
