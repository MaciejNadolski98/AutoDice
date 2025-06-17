use bevy::{prelude::*, render::camera::ScalingMode};
use crate::{constants::{HEIGHT, WIDTH}, states::GameState};

#[derive(Component)]
pub struct MenuCamera;

pub struct MenuCameraPlugin;

impl Plugin for MenuCameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Loading), spawn_menu_camera)
      .add_systems(OnExit(GameState::Loading), despawn_menu_camera)
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
    Projection::from(OrthographicProjection {
      scaling_mode: ScalingMode::AutoMin {
        min_width: WIDTH,
        min_height: HEIGHT,
      },
      ..OrthographicProjection::default_2d()
    }),
    MenuCamera,
  ));
}

fn despawn_menu_camera(
  mut commands: Commands,
  camera_: Query<Entity, With<MenuCamera>>,
) {
  let camera = camera_.single().unwrap();
  commands.entity(camera).despawn();
}
