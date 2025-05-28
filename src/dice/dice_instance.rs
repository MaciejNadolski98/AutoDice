use bevy::prelude::*;
use avian3d::prelude::*;

use crate::constants::MAX_DICE_COUNT;
use crate::dice::events::SpawnDices;
use crate::manage::plugin::DiceData;
use crate::states::GameState;

use super::dice_render::{
  build_dices,
  DiceFaceImage
};
use super::dice_template::DiceTemplate;
use super::{ChangeDiceFace, FaceDescription};
use std::collections::HashMap;

pub struct DiceInstancePlugin;

impl Plugin for DiceInstancePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(DiceEntityMap::default())
      .add_systems(OnEnter(GameState::Battle), spawn_dices)
      .add_systems(OnExit(GameState::Battle), despawn_dices);
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DiceID {
  pub team_id: usize,
  pub dice_id: usize,
}

#[derive(Component, Default)]
pub struct Dice {
  id: DiceID,
  max_hp: u32,
  current_hp: u32,
  current_faces: [FaceDescription; 6],
}

impl Dice {
  pub fn build(
    dice_face_changed: &mut EventWriter<ChangeDiceFace>,
    template: DiceTemplate, 
    dice_id: DiceID,
  ) -> Self {
    let mut dice = Dice::default();
    dice.set_id(dice_id);
    dice.set_max_hp(template.hp);
    dice.set_current_hp(template.hp);
    for i in 0..6 {
      dice_face_changed.send(dice.set_face(i, template.faces[i]));
    }
    dice
  }

  pub fn id(&self) -> DiceID {
    self.id
  }

  pub fn max_hp(&self) -> u32 {
    self.max_hp
  }

  pub fn current_hp(&self) -> u32 {
    self.current_hp
  }

  pub fn face(&self, face_id: usize) -> FaceDescription {
    self.current_faces[face_id]
  }

  pub fn faces(&self) -> &[FaceDescription; 6] {
    &self.current_faces
  }

  pub fn set_id(&mut self, id: DiceID) {
    self.id = id;
  }

  pub fn set_max_hp(&mut self, max_hp: u32) {
    self.max_hp = max_hp;
  }
  
  pub fn set_current_hp(&mut self, current_hp: u32) {
    self.current_hp = current_hp;
  }

  pub fn set_face(&mut self, face_id: usize, face: FaceDescription) -> ChangeDiceFace {
    self.current_faces[face_id] = face;
    ChangeDiceFace { dice_id: self.id, face_id: face_id, face: face }
  }
}

#[derive(Resource, Default)]
pub struct DiceEntityMap(pub HashMap<DiceID, Entity>);

fn spawn_dices(
  dice_data: Res<DiceData>,
  meshes: ResMut<Assets<Mesh>>,
  mut commands: Commands,
  mut materials: ResMut<Assets<StandardMaterial>>,
  dice_face_image: Res<DiceFaceImage>,
  mut dice_face_changed: EventWriter<ChangeDiceFace>,
  mut dice_spawn_event: EventWriter<SpawnDices>,
  mut dice_entity_map: ResMut<DiceEntityMap>,
) {
  assert!(dice_data.team1.len() <= MAX_DICE_COUNT);
  assert!(dice_data.team2.len() <= MAX_DICE_COUNT);
  
  let dice_meshes = build_dices(meshes);

  for i in 0..dice_data.team1.len() {
    let dice_id = DiceID { team_id: 0, dice_id: i };
    let entity = commands.spawn((
      Name::new(format!("Red dice {}", i+1)),
      Mesh3d(dice_meshes[0][i].clone()),
      MeshMaterial3d(materials.add(StandardMaterial { base_color_texture: Some(dice_face_image.image.clone()), ..default()})),
      RigidBody::Dynamic,
      Collider::cuboid(1.0, 1.0, 1.0),
      Dice::build(&mut dice_face_changed, dice_data.team1[i].clone(), dice_id),
    )).id();
    dice_entity_map.0.insert(dice_id, entity);
  }

  for i in 0..dice_data.team2.len() {
    let dice_id = DiceID { team_id: 1, dice_id: i };
    let entity = commands.spawn((
      Name::new(format!("Red dice {}", i+1)),
      Mesh3d(dice_meshes[0][i].clone()),
      MeshMaterial3d(materials.add(StandardMaterial { base_color_texture: Some(dice_face_image.image.clone()), ..default()})),
      RigidBody::Dynamic,
      Collider::cuboid(1.0, 1.0, 1.0),
      Dice::build(&mut dice_face_changed, dice_data.team2[i].clone(), dice_id),
    )).id();
    dice_entity_map.0.insert(dice_id, entity);
  }
  dice_spawn_event.send(SpawnDices);
}

fn despawn_dices(
  mut commands: Commands,
  entities: Query<Entity, With<Dice>>,
  mut dice_entity_map: ResMut<DiceEntityMap>
) {
  for entity in &entities {
    commands.entity(entity).despawn();
  }
  dice_entity_map.0.clear();
}
