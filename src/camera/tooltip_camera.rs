use bevy::{prelude::*, render::camera::ScalingMode};
use crate::constants::{HEIGHT, TOOLTIP_LAYER, WIDTH};

#[derive(Component)]
struct TooltipCamera;

pub struct TooltipCameraPlugin;

impl Plugin for TooltipCameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_tooltip_camera);
  }
}

fn spawn_tooltip_camera(
  mut commands: Commands,
) {
  commands.spawn((
    Name::new("Menu camera"),
    Camera2d,
    Camera {
      order: 2,
      ..default()
    },
    Transform::from_translation(Vec3::Z * 100.0),
    Projection::from(OrthographicProjection {
      scaling_mode: ScalingMode::AutoMin {
        min_width: WIDTH,
        min_height: HEIGHT,
      },
      ..OrthographicProjection::default_2d()
    }),
    TooltipCamera,
    TOOLTIP_LAYER,
  ));
}
