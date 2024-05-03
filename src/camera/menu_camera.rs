use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct MenuCamera;

pub fn spawn_menu_camera(
  mut commands: Commands,
) {
  let mut camera = Camera2dBundle::default();

  camera.projection.scaling_mode = ScalingMode::AutoMin {
      min_width: 256.0,
      min_height: 144.0,
  };
  commands.spawn((
    Camera2dBundle {
      projection: OrthographicProjection {
        scaling_mode: ScalingMode::AutoMin {
          min_width: 256.0,
          min_height: 144.0,
        },
        ..default()
      },
      ..default()
    },
    MenuCamera,
  ));
}

pub fn despawn_menu_camera(
  mut commands: Commands,
  camera_: Query<Entity, With<MenuCamera>>,
) {
  let camera = camera_.single();
  commands.entity(camera).despawn();
}
