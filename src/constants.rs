use bevy::render::view::RenderLayers;


pub const WIDTH: f32 = 256.0;
pub const HEIGHT: f32 = 144.0;

pub const WALL_SIZE: f32 = 10_000.0;

pub const DICE_SIZE: f32 = 10.0;
pub const DICE_TEXTURE_SIZE: f32 = 32.0;

pub const DEFAULT_CAMERA_DISTANCE: f32 = 200.0;
pub const MAX_CAMERA_DISTANCE: f32 = 10_000.0;

// Dice are assumed to be around the size of 1 centimeter
pub const GRAVITY_ACCELERATION: f32 = 9.81 * DICE_SIZE * 10.0;

pub const MAX_DICE_COUNT: usize = 5;

pub const DICE_FACES_LAYER: RenderLayers = RenderLayers::layer(1);
