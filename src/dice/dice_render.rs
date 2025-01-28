use std::array;

use bevy::{prelude::*, render::render_resource::Extent3d};
use bevy::render::render_resource::{TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

use super::events::{DiceFaceChangedEvent, ActionType};
use crate::constants::{MAX_DICE_COUNT, DICE_TEXTURE_SIZE, DICE_FACES_LAYER};

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
  fn get(&self, team_id: u32, dice_id: u32, face_id: u32) -> Entity {
    self.array[team_id as usize][dice_id as usize][face_id as usize]
  }
}

fn spawn_dice_faces(
  mut commands: Commands,
) {
  let array = [0, 1].map(|team_id| {
    let ret: [_; MAX_DICE_COUNT] = array::from_fn(|dice_id| {
      let ret: [_; 6] = array::from_fn(|face_id| {
        let [x, y] = get_uv(team_id, dice_id as u32, face_id as u32);
        let [width, height] = [6.0 * DICE_TEXTURE_SIZE, 2.0 * DICE_TEXTURE_SIZE * (MAX_DICE_COUNT as f32)];
        let [abs_x, abs_y] = [x * width + 0.5 * DICE_TEXTURE_SIZE, y * height + 0.5 * DICE_TEXTURE_SIZE];
        let [center_x, center_y] = [abs_x - width / 2.0, height / 2.0 - abs_y];
        commands.spawn((
          SpatialBundle {
            transform: Transform::from_xyz(center_x, center_y, 0.0),
            visibility: Visibility::Visible,
            ..default()
          },
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
  match vertex_id {
    0 => [x, y],
    1 => [x+dice_size[0], y],
    2 => [x+dice_size[0], y+dice_size[1]],
    3 => [x, y+dice_size[1]],
    _ => panic!("Invali vertex_id value"),
  }
}

// TODDO make private
#[derive(Default, Resource)]
pub struct DiceFaceImage {
  pub image: Handle<Image>,
}

fn spawn_dice_camera(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
) {
  let size = Extent3d {
    width: 6u32 * (DICE_TEXTURE_SIZE as u32),
    height: 2u32 * (MAX_DICE_COUNT as u32) * (DICE_TEXTURE_SIZE as u32),
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
    Camera2dBundle {
      camera: Camera {
        order: -1,
        target: image_handle.clone().into(),
        ..default()
      },
      ..default()
    },
    DICE_FACES_LAYER,
  ));
  commands.insert_resource(DiceFaceImage { image: image_handle });
}

fn update_dice_faces(
  mut commands: Commands,
  mut events: EventReader<DiceFaceChangedEvent>,
  entities: Res<FaceMatrix>,
  asset_server: Res<AssetServer>
) {
  for face_update in events.read() {
    assert!(face_update.team_id <= 1);
    assert!(face_update.dice_id < MAX_DICE_COUNT as u32);
    assert!(face_update.face_id < 6);
    let texture = match face_update.face.action_type {
      ActionType::Attack => asset_server.load("sword.png"),
      ActionType::Heal => asset_server.load("heal.png"),
      ActionType::Defend => asset_server.load("shield.png"),
    };
    let face_entity = entities.get(face_update.team_id, face_update.dice_id, face_update.face_id);
    
    commands.entity(face_entity)
      .despawn_descendants()
      .with_children(|commands| {
        commands.spawn((
          SpriteBundle {
            texture: texture,
            ..default()
          },
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
      meshes.add(Cuboid::default().mesh().with_removed_attribute(Mesh::ATTRIBUTE_UV_0).with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![
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
