use std::array;

use bevy::{prelude::*, render::render_resource::Extent3d};
use bevy::render::render_resource::{TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

use super::events::ChangeDiceFace;
use super::DiceID;
use crate::constants::{DICE_FACES_LAYER, MAX_DICE_COUNT};
use crate::constants::dice_texture;
use crate::dice::action::Action;

pub struct DiceRenderPlugin;

impl Plugin for DiceRenderPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, update_dice_faces)
      .add_systems(Startup, (spawn_dice_camera, spawn_dice_faces));
  }
}

#[derive(Resource)]
struct FaceMatrix {
  pub array: [
    [
      [Entity; 6];
      MAX_DICE_COUNT
    ];
    2
  ],
}

impl FaceMatrix {
  fn get(&self, dice_id: DiceID, face_id: usize) -> Entity {
    self.array[dice_id.team_id][dice_id.dice_id][face_id]
  }
}

fn spawn_dice_faces(
  mut commands: Commands,
) {
  let array = [0, 1].map(|team_id| {
    let ret: [_; MAX_DICE_COUNT] = array::from_fn(|dice_id| {
      let ret: [_; 6] = array::from_fn(|face_id| {
        let [x, y] = get_uv(team_id, dice_id as u32, face_id as u32);
        let [width, height] = [6.0 * dice_texture::TARGET_SIZE, 2.0 * dice_texture::TARGET_SIZE * (MAX_DICE_COUNT as f32)];
        let [abs_x, abs_y] = [x * width + 0.5 * dice_texture::TARGET_SIZE, y * height + 0.5 * dice_texture::TARGET_SIZE];
        let [center_x, center_y] = [abs_x - width / 2.0, height / 2.0 - abs_y];
        commands.spawn((
          Name::new("Dice face"),
          Transform::from_xyz(center_x, center_y, 0.0),
          Visibility::Visible,
          DICE_FACES_LAYER,
        )).id()
      });
      ret
    });
    ret
  });

  commands.insert_resource(FaceMatrix { array });
}

fn get_uv_dice_size() -> [f32; 2] {
  [1.0 / 6.0, 1.0 / (2.0 * MAX_DICE_COUNT as f32)]
}

fn get_uv(team_id: u32, dice_id: u32, face_id: u32) -> [f32; 2] {
  let [w, h] = get_uv_dice_size();
  [
    w * face_id as f32, 
    h * (team_id + 2 * dice_id) as f32,
  ]
}

fn get_uv_vertex(team_id: u32, dice_id: u32, face_id: u32, vertex_id: u32) -> [f32; 2] {
  let [x, y] = get_uv(team_id, dice_id, face_id);
  let dice_size = get_uv_dice_size();
  match permute_vertex_id(face_id, vertex_id) {
    0 => [x, y],
    1 => [x, y+dice_size[1]],
    2 => [x+dice_size[0], y+dice_size[1]],
    3 => [x+dice_size[0], y],
    _ => panic!("Invali vertex_id value"),
  }
}

fn permute_vertex_id(face_id: u32, vertex_id: u32) -> u32 {
  match face_id {
    0 => (vertex_id + 1) % 4,
    1 => (vertex_id + 3) % 4,
    2 => (vertex_id + 2) % 4,
    3 => (vertex_id + 2) % 4,
    4 => (vertex_id + 3) % 4,
    5 => (vertex_id + 3) % 4,
    _ => panic!("Invalid face_id value"),
  }
}

#[derive(Default, Resource)]
pub struct DiceFaceImage {
  pub image: Handle<Image>,
}

fn spawn_dice_camera(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
) {
  let size = Extent3d {
    width: 6 * (dice_texture::TARGET_SIZE as u32),
    height: 2 * (MAX_DICE_COUNT as u32) * (dice_texture::TARGET_SIZE as u32),
    ..default()
  };

  let mut image = Image {
    texture_descriptor: TextureDescriptor {
      label: None,
      size,
      dimension: TextureDimension::D2,
      format: TextureFormat::Bgra8UnormSrgb,
      mip_level_count: 1,
      sample_count: 1,
      usage: TextureUsages::TEXTURE_BINDING
          | TextureUsages::COPY_DST
          | TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[],
    },
    ..default()
  };
  image.resize(size);

  let image_handle = images.add(image);

  commands.spawn((
    Name::new("Dice texture camera"),
    Camera2d,
    Camera {
      order: -1,
      target: image_handle.clone().into(),
      ..default()
    },
    DICE_FACES_LAYER,
  ));
  commands.insert_resource(DiceFaceImage { image: image_handle });
}

fn update_dice_faces(
  mut commands: Commands,
  mut events: EventReader<ChangeDiceFace>,
  entities: Res<FaceMatrix>,
  asset_server: Res<AssetServer>
) {
  for face_update in events.read() {
    assert!(face_update.dice_id.team_id <= 1);
    assert!(face_update.dice_id.dice_id < MAX_DICE_COUNT);
    assert!(face_update.face_id < 6);
    let texture = match face_update.face.action {
      Action::Attack => asset_server.load("actions/axe.png"),
      Action::Regenerate => asset_server.load("actions/heart.png"),
      Action::Defend => asset_server.load("actions/shield.png"),
      Action::Fire => asset_server.load("actions/fire.png"),
      _ => panic!("Invalid action type"),
    };
    let face_entity = entities.get(face_update.dice_id, face_update.face_id);
    
    commands.entity(face_entity)
      .despawn_related::<Children>()
      .with_children(|commands| {
        commands.spawn((
          Name::new("Face image"),
          Sprite::from_image(texture),
          Transform::default()
            .with_scale(Vec3::splat(dice_texture::SCALING_FACTOR))
            .with_translation((dice_texture::OFFSET, 1.0).into()),
          DICE_FACES_LAYER,
        ));
        commands.spawn((
          Name::new("Face background"),
          Sprite::from_color(Color::linear_rgb(0.2, 0.2, 0.2), Vec2::splat(dice_texture::TARGET_SIZE)),
          DICE_FACES_LAYER,
        )).with_children(|commands| {
          commands.spawn((
            Name::new("Inside"),
            Sprite::from_color(Color::WHITE, Vec2::splat(dice_texture::INNER_SIZE)),
            Transform::default(),
            DICE_FACES_LAYER,
          ));
        });
        commands.spawn((
          Name::new("Pips"),
          Text2d(format!("{}", face_update.face.pips_count)),
          TextFont {
            font_size: dice_texture::FONT_SIZE,
            ..default()
          },
          TextColor::BLACK,
          Transform::from_translation((dice_texture::PIPS_POSITION, 1.0).into()),
          DICE_FACES_LAYER,
        ));
      });
  }
}

pub fn build_dices(
  mut meshes: ResMut<Assets<Mesh>>
) -> [std::vec::Vec<bevy::prelude::Handle<bevy::prelude::Mesh>>; 2] {
  [0, 1].map(|team_id| {
    (0..MAX_DICE_COUNT).map(|dice_id| {
      meshes.add(Cuboid::default().mesh().build().with_removed_attribute(Mesh::ATTRIBUTE_UV_0).with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![
        // Front
        get_uv_vertex(team_id, dice_id as u32, 0, 0),
        get_uv_vertex(team_id, dice_id as u32, 0, 1),
        get_uv_vertex(team_id, dice_id as u32, 0, 2),
        get_uv_vertex(team_id, dice_id as u32, 0, 3),
        // Back
        get_uv_vertex(team_id, dice_id as u32, 1, 0),
        get_uv_vertex(team_id, dice_id as u32, 1, 1),
        get_uv_vertex(team_id, dice_id as u32, 1, 2),
        get_uv_vertex(team_id, dice_id as u32, 1, 3),
        // Right
        get_uv_vertex(team_id, dice_id as u32, 2, 0),
        get_uv_vertex(team_id, dice_id as u32, 2, 1),
        get_uv_vertex(team_id, dice_id as u32, 2, 2),
        get_uv_vertex(team_id, dice_id as u32, 2, 3),
        // Left
        get_uv_vertex(team_id, dice_id as u32, 3, 0),
        get_uv_vertex(team_id, dice_id as u32, 3, 1),
        get_uv_vertex(team_id, dice_id as u32, 3, 2),
        get_uv_vertex(team_id, dice_id as u32, 3, 3),
        // Top
        get_uv_vertex(team_id, dice_id as u32, 4, 0),
        get_uv_vertex(team_id, dice_id as u32, 4, 1),
        get_uv_vertex(team_id, dice_id as u32, 4, 2),
        get_uv_vertex(team_id, dice_id as u32, 4, 3),
        // Bottom
        get_uv_vertex(team_id, dice_id as u32, 5, 0),
        get_uv_vertex(team_id, dice_id as u32, 5, 1),
        get_uv_vertex(team_id, dice_id as u32, 5, 2),
        get_uv_vertex(team_id, dice_id as u32, 5, 3),
      ]))
  }).collect::<Vec<_>>()})
}
