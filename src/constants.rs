use avian3d::math::PI;
use bevy::{math::Vec3, render::view::RenderLayers};

pub const BASE_SCALE: f32 = 10.0;

pub const WIDTH: f32 = 256.0 * BASE_SCALE;
pub const HEIGHT: f32 = 144.0 * BASE_SCALE;

pub const WALL_SIZE: f32 = 10_000.0 * BASE_SCALE;

pub const DICE_SIZE: f32 = 10.0 * BASE_SCALE;

pub const GRID_FACE_SIZE: f32 = 0.5 * DICE_SIZE;

pub mod ui {
  use bevy::ui::Val;
  use crate::constants::BASE_SCALE;

  pub const BUTTON_SIZE: Val = Val::Px(BASE_SCALE * 20.0);
  pub const ROUND_NUMBER_SIZE: f32 = BASE_SCALE * 2.5;
}

pub mod dice_texture {
  use bevy::math::Vec2;

  pub const SOURCE_SIZE: f32 = 16.0;
  pub const SCALING_FACTOR: f32 = 4.0;

  pub const SCALED_SOURCE_SIZE: f32 = SOURCE_SIZE * SCALING_FACTOR;
  pub const INNER_SIZE: f32 = SCALED_SOURCE_SIZE + EXTRA_INNER_SPACE + INNER_MARGIN * 2.0;
  pub const EXTRA_INNER_SPACE: f32 = 8.0;
  pub const INNER_MARGIN: f32 = 1.0;
  pub const OFFSET: Vec2 = Vec2::new(-EXTRA_INNER_SPACE / 2.0, EXTRA_INNER_SPACE / 2.0);

  pub const FRAME_WIDTH: f32 = 3.0;
  pub const TARGET_SIZE: f32 = INNER_SIZE + FRAME_WIDTH * 2.0;

  pub const FONT_SIZE: f32 = 20.0;
  pub const PIPS_POSITION: Vec2 = Vec2::new(TARGET_SIZE / 2.8, -TARGET_SIZE / 2.8);
}

pub mod dice_info_bar {
  use bevy::math::{Vec2, Vec3};

  use crate::constants::DICE_SIZE;

  pub const STATUS_MARGIN: f32 = 0.05 * DICE_SIZE;
  pub const BAR_DISPLACEMENT: f32 = -0.8 * DICE_SIZE;

  pub const STATUS_BAR_POSITION: Vec3 = Vec3::new(0.0, BAR_DISPLACEMENT, 0.0);
  pub const STATUS_ICON_SIZE: Vec2 = Vec2::splat(0.25 * DICE_SIZE);
  pub const STATUS_TEXT_SIZE: f32 = 0.25 * DICE_SIZE;

  pub const HEALTH_BAR_POSITION: Vec3 = Vec3::new(0.0, BAR_DISPLACEMENT - STATUS_ICON_SIZE.y - STATUS_MARGIN, 0.0);
  pub const HEALTH_BAR_WIDTH: f32 = 1.0 * DICE_SIZE;
  pub const HEALTH_BAR_HEIGHT: f32 = 0.25 * DICE_SIZE;
}

pub mod loading_screen {
  use bevy::math::Vec2;

  use crate::constants::{BASE_SCALE, HEIGHT, WIDTH};

  pub const MARGIN: f32 = 1.0 * BASE_SCALE;
  pub const INNER_BAR_SIZE: Vec2 = Vec2::new(0.3 * WIDTH, 0.05 * HEIGHT);
  pub const BAR_SIZE: Vec2 = Vec2::new(INNER_BAR_SIZE.x + MARGIN, INNER_BAR_SIZE.y + MARGIN);
}

pub const DEFAULT_CAMERA_DISTANCE: f32 = 200.0 * BASE_SCALE;
pub const MAX_CAMERA_DISTANCE: f32 = 800.0 * BASE_SCALE;
pub const CAMERA_SWAP_TIME: f32 = 0.5;

// Dice are assumed to be around the size of 1 centimeter
pub const GRAVITY_ACCELERATION: f32 = 9.81 * DICE_SIZE * 10.0;

pub const DICE_COUNT: usize = 5;
pub const SHOP_ITEMS_COUNT: usize = 4;

pub const DICE_FACES_LAYER: RenderLayers = RenderLayers::layer(1);
pub const BATTLE_OVERLAY_LAYER: RenderLayers = RenderLayers::layer(2);
pub const TOOLTIP_LAYER: RenderLayers = RenderLayers::layer(3);

pub const RESOLUTION_WIDTH: f32 = 1280.0;
pub const RESOLUTION_HEIGHT: f32 = 720.0;

pub const LINEAR_VELOCITY_EPSILON: f32 = 1.0 * BASE_SCALE;
pub const ANGULAR_VELOCITY_EPSILON: f32 = 1.0;

pub const LINEAR_SPEED: f32 = DICE_SIZE * 10.0;
pub const ANGULAR_SPEED: f32 = 2.0 * (2.0 * PI);

pub const FACE_NORMALS: [Vec3; 6] = [
    Vec3::Z,
    Vec3::NEG_Z,
    Vec3::X,
    Vec3::NEG_X,
    Vec3::Y,
    Vec3::NEG_Y,
];

pub const FLOATING_TEXT_DURATION: f32 = 3.0;
pub const FLOATING_TEXT_SPEED: f32 = DICE_SIZE;
pub const FLOATING_TEXT_FONT_SIZE: f32 = DICE_SIZE / 2.0;
