use avian3d::prelude::{Collider, RigidBody, RigidBodyDisabled};
use bevy::render::mesh::{MeshVertexAttribute, MeshVertexBufferLayouts, VertexAttributeValues, VertexFormat};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::prelude::*;

use crate::constants::DICE_SIZE;
use crate::dice::dice_instance::DiceEntityMap;
use crate::dice::{Dice, DiceID, DiceTemplate, Face};

pub struct DiceRenderPlugin;

impl Plugin for DiceRenderPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<MeshVertexBufferLayouts>()
      .add_plugins(MaterialPlugin::<DiceMaterial>::default());
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

#[derive(Component)]
pub struct DiceCamera;

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
  input: In<(DiceID, Entity)>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut images: ResMut<Assets<Image>>,
  mut commands: Commands,
  mut materials: ResMut<Assets<DiceMaterial>>,
  mut dice_entity_map: ResMut<DiceEntityMap>,
  templates: Query<&DiceTemplate>,
  faces: Query<&Face>,
  children: Query<&Children>,
) {
  let (dice_id, template) = *input;
  let mut face_vector = Vec::new();
  let mut image_vector = Vec::new();
  for &face_template in children.get(template).unwrap() {
    let face = Face::from_other(faces.get(face_template).unwrap(), &mut images);
    face_vector.push(face.clone());
    image_vector.push(face.image.clone());
  }

  let mesh = DiceMeshBuilder.build();
  let handle = meshes.add(mesh.clone());

  let dice_entity = commands.spawn((
      Name::new("Dice instance"),
      Mesh3d(handle),
      MeshMaterial3d(materials.add(DiceMaterial::new(image_vector))),
      RigidBody::Dynamic,
      Collider::cuboid(1.0, 1.0, 1.0),
      Dice::build(templates.get(template).unwrap().clone(), dice_id),
      Transform::from_translation(Vec3::new(0.0, 0.0, DICE_SIZE * 0.5))
        .with_scale(Vec3::splat(DICE_SIZE)),
      RigidBodyDisabled,
    ))
    .with_children(|commands| {
      for face in face_vector {
        commands.spawn(face);
      }
    })
    .id();

  dice_entity_map.0.insert(dice_id, dice_entity);
}
