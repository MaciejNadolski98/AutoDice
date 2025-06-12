use std::collections::HashMap;

use avian3d::prelude::{Collider, RigidBody};
use bevy::render::mesh::{MeshVertexAttribute, MeshVertexBufferLayouts, VertexAttributeValues, VertexFormat};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::prelude::*;

use crate::constants::DICE_FACES_LAYER;
use crate::constants::dice_texture::{FONT_SIZE, INNER_SIZE, OFFSET, PIPS_POSITION, SCALING_FACTOR, TARGET_SIZE};
use crate::dice::dice_instance::DiceEntityMap;
use crate::dice::dice_template::Face;
use crate::dice::Dice;
use crate::loading_screen::AssetStore;

pub struct DiceRenderPlugin;

impl Plugin for DiceRenderPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<MapHandleToRootAndCamera>()
      .init_resource::<MeshVertexBufferLayouts>()
      .add_plugins(MaterialPlugin::<DiceMaterial>::default())
      .add_systems(First, deactivate_dice_cameras);
  }
}

fn permute_vertex_id(face_id: usize, vertex_id: usize) -> usize {
  match face_id {
    0 => (7 - vertex_id) % 4,
    1 => (vertex_id + 0) % 4,
    2 => (6 - vertex_id) % 4,
    3 => (vertex_id + 3) % 4,
    4 => (vertex_id + 0) % 4,
    5 => (5 - vertex_id) % 4,
    _ => panic!("Invalid face_id value"),
  }
}

#[derive(Resource, Default)]
pub struct MapHandleToRootAndCamera {
  counter: usize,
  mapping: HashMap<Handle<Image>, (Entity, Entity)>,
}

#[derive(Component)]
pub struct DiceCamera;

impl MapHandleToRootAndCamera {
  fn add(&mut self, image: Handle<Image>, commands: &mut Commands) {
    let position = TARGET_SIZE * Vec2::X * self.counter as f32;
    self.counter += 1;
    let root = commands.spawn((
      Name::new("Dice face root"),
      Visibility::Visible,
      Transform::from_translation(Vec3::from((position, 0.0))),
      DICE_FACES_LAYER,
    )).id();
    let camera = commands
      .spawn((
        Name::new("Dice texture camera"),
        Camera2d,
        DiceCamera,
        Camera {
          order: -1,
          target: image.clone().into(),
          ..default()
        },
        Transform::from_translation(Vec3::from((position, 100.0))).looking_at(Vec3::from((position, 0.0)), Vec3::Y),
        DICE_FACES_LAYER,
      )).id();
    self.mapping.insert(
      image.clone_weak(),
      (root, camera),
    );
  }

  fn get(&self, image: Handle<Image>) -> (Entity, Entity) {
    *self.mapping.get(&image).unwrap()
  }

  pub fn remove(&mut self, image: Handle<Image>, commands: &mut Commands) {
    let (root, camera) = self.mapping.get(&image).unwrap();
    commands.entity(*root).despawn();
    commands.entity(*camera).despawn();
    self.mapping.remove(&image);
  }
}

pub fn spawn_dice_camera(
  image: In<Handle<Image>>,
  mut commands: Commands,
  mut map_handle_to_root: ResMut<MapHandleToRootAndCamera>,
) {
  map_handle_to_root.add(image.clone(), &mut commands);
}

pub fn despawn_face(
  image: In<Handle<Image>>,
  mut commands: Commands,
  mut map_handle_to_root: ResMut<MapHandleToRootAndCamera>,
) {
  map_handle_to_root.remove(image.clone(), &mut commands);
}

pub fn update_dice_face(
  input: In<(Face, Handle<Image>)>,
  asset_store: Res<AssetStore>,
  map_handle_to_root: Res<MapHandleToRootAndCamera>,
  mut commands: Commands,
  mut cameras: Query<&mut Camera>,
) {
  let (face, target_image) = input.clone();
  let action_image = asset_store.get(face.action.into());

  let (root, camera) = map_handle_to_root.get(target_image.clone());
  commands
    .entity(root)
    .despawn_related::<Children>()
    .with_children(|commands| {
      commands.spawn((
        Name::new("Background"),
        Sprite::from_color(
          Color::linear_rgb(0.2, 0.2, 0.2),
          Vec2::splat(TARGET_SIZE)
        ),
        DICE_FACES_LAYER,
      )).with_children(|commands| {
        commands.spawn((
          Name::new("Foreground"),
          Transform::from_translation(Vec3::Z),
          Sprite::from_color(
            Color::WHITE,
            Vec2::splat(INNER_SIZE),
          ),
          DICE_FACES_LAYER,
        )).with_children(|commands| {
          commands.spawn((
            Name::new("Face icon"),
            Sprite::from_image(action_image),
            Transform::default()
              .with_scale(Vec3::splat(SCALING_FACTOR))
              .with_translation((OFFSET, 3.0).into()),
            DICE_FACES_LAYER,
          ));

          commands.spawn((
            Name::new("Pips"),
            Text2d(format!("{}", face.pips_count)),
            TextFont {
              font_size: FONT_SIZE,
              ..default()
            },
            TextColor::BLACK,
            Transform::from_translation((PIPS_POSITION, 1.0).into()),
            DICE_FACES_LAYER,
          ));
        });
      });
    });
  cameras.get_mut(camera).unwrap().is_active = true;
}

fn deactivate_dice_cameras(
  cameras: Query<&mut Camera, (Changed<Camera>, With<DiceCamera>)>,
) {
  for mut camera in cameras {
    camera.is_active = false;
  }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct DiceMaterial {
  #[texture(0)] #[sampler(1)]
  texture0: Handle<Image>,
  #[texture(2)] #[sampler(3)]
  texture1: Handle<Image>,
  #[texture(4)] #[sampler(5)]
  texture2: Handle<Image>,
  #[texture(6)] #[sampler(7)]
  texture3: Handle<Image>,
  #[texture(8)] #[sampler(9)]
  texture4: Handle<Image>,
  #[texture(10)] #[sampler(11)]
  texture5: Handle<Image>,
}

impl DiceMaterial {
  pub fn new<T>(textures: T) -> Self
  where
    T: IntoIterator<Item = Handle<Image>>,
  {
    let textures: Vec<_> = textures.into_iter().collect();
    assert!(textures.len() == 6, "At least 6 textures are required");
    Self {
      texture0: textures[0].clone(),
      texture1: textures[1].clone(),
      texture2: textures[2].clone(),
      texture3: textures[3].clone(),
      texture4: textures[4].clone(),
      texture5: textures[5].clone(),
    }
  }
}

impl Material for DiceMaterial {

  fn vertex_shader() -> ShaderRef {
    "shaders/dice.wgsl".into()
  }

  fn fragment_shader() -> ShaderRef {
    "shaders/dice.wgsl".into()
  }
}

const ATTRIBUTE_FACE_INDEX: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Face_Index", 3, VertexFormat::Uint32);

pub struct DiceMeshBuilder;

impl MeshBuilder for DiceMeshBuilder {
  fn build(&self) -> Mesh {
    let mut mesh = Cuboid::default()
      .mesh()
      .build()
      .with_inserted_attribute(ATTRIBUTE_FACE_INDEX,
        (0..6)
          .map(|face_index| {
            vec![face_index, face_index, face_index, face_index]
          })
          .into_iter()
          .flatten()
          .collect::<Vec<u32>>()
      );
    let VertexAttributeValues::Float32x2(uvs) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap() else { panic!() };
    let mut new_uvs = Vec::new();
    let mut i = 0;
    for face_id in 0..6 {
      for vertex_id in 0..4 {
        let swap_with = i - vertex_id + permute_vertex_id(face_id, vertex_id);
        new_uvs.push(uvs[swap_with]);
        i += 1;
      }
    }
    *uvs = new_uvs;
    mesh
  }
}

pub fn spawn_dice(
  dice: In<Dice>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut commands: Commands,
  mut materials: ResMut<Assets<DiceMaterial>>,
  mut dice_entity_map: ResMut<DiceEntityMap>,
) {
  let mesh = DiceMeshBuilder.build();
  let handle = meshes.add(mesh.clone());

  let images = dice.faces().map(|(_, image)| {image});
  let entity = commands.spawn((
    Name::new("Dice instance"),
    Mesh3d(handle),
    MeshMaterial3d(materials.add(DiceMaterial::new(images))),
    RigidBody::Dynamic,
    Collider::cuboid(1.0, 1.0, 1.0),
    dice.clone(),
  )).id();

  dice_entity_map.0.insert(dice.id(), entity);
}
