use bevy::prelude::*;

use crate::constants::dice_texture::INNER_SIZE;

pub struct FaceBackgroundPlugin;

impl Plugin for FaceBackgroundPlugin {
  fn build(&self, _app: &mut App) {
  }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FaceBackground {
  #[default]
  Empty,
  Cruel,
  Double,
}

impl From<FaceBackground> for Sprite {
  fn from(background: FaceBackground) -> Self {
    match background {
      FaceBackground::Empty => Sprite {
        color: Color::WHITE,
        custom_size: Some(Vec2::splat(INNER_SIZE)),
        ..default()
      },
      FaceBackground::Cruel => Sprite {
        color: Color::linear_rgb(0.8, 0.5, 0.5),
        custom_size: Some(Vec2::splat(INNER_SIZE)),
        ..default()
      },
      FaceBackground::Double => Sprite {
        color: Color::linear_rgb(0.5, 0.8, 0.5),
        custom_size: Some(Vec2::splat(INNER_SIZE)),
        ..default()
      },
    }
  }
}
