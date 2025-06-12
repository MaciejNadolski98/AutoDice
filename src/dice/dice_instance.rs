use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy_defer::{fetch, AccessError, AsyncAccess, AsyncWorld};

use crate::constants::dice_texture::TARGET_SIZE;
use crate::constants::MAX_DICE_COUNT;
use crate::dice::dice_render::{despawn_face, spawn_dice_camera, update_dice_face};
use crate::dice::events::SpawnDices;
use crate::manage::plugin::DiceData;
use crate::states::GameState;
use crate::utils::*;

use super::animation::get_dice_entity;
use super::dice_render::spawn_dice;
use super::dice_template::{DiceTemplate, Face};
use super::events::DiceDied;
use super::roll::get_face_id;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct DiceInstancePlugin;

impl Plugin for DiceInstancePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(DiceEntityMap::default())
      .insert_resource(Rows::default())
      .add_systems(OnEnter(GameState::Battle), spawn_dices)
      .add_systems(OnExit(GameState::Battle), despawn_dices)
      .register_listener(despawn_dead_dice);
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DiceID {
  pub team_id: usize,
  pub dice_id: usize,
}

#[derive(Component, Default, Clone)]
#[component(on_remove = free_dice_faces)]
pub struct Dice {
  id: DiceID,
  max_hp: u32,
  current_hp: u32,
  current_faces: [(Face, Handle<Image>); 6],
  row_position: usize,
}

fn free_dice_faces(
  mut world: DeferredWorld,
  context: HookContext,
) {
  let entity = context.entity;
  let dice = world.get::<Dice>(entity).unwrap().clone();

  for (_, handle) in dice.faces() {
    world.commands().run_system_cached_with(despawn_face, handle);
  }
}

impl Dice {
  pub fn build(
    template: DiceTemplate, 
    dice_id: DiceID,
    images: &mut Assets<Image>,
  ) -> Self {
    let mut dice = Dice::default();
    dice.set_id(dice_id);
    dice.set_max_hp(template.hp);
    dice.set_current_hp(template.hp);
    for i in 0..6 {
      let size = Extent3d {
        width: TARGET_SIZE as u32,
        height: TARGET_SIZE as u32,
        depth_or_array_layers: 1,
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
            | TextureUsages::RENDER_ATTACHMENT,
          view_formats: &[],
        },
        ..default()
      };
      image.resize(size);
      dice.current_faces[i] = (template.faces[i], images.add(image));
    }
    dice.set_row_position(dice_id.dice_id);
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

  pub fn faces(&self) -> [(Face, Handle<Image>); 6] {
    self.current_faces.clone()
  }

  pub fn face(&self, face_id: usize) -> Face {
    self.current_faces[face_id].0
  }

  pub fn row_position(&self) -> usize {
    self.row_position
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

  pub fn set_row_position(&mut self, row_position: usize) {
    self.row_position = row_position;
  }
}

#[derive(Resource, Default)]
pub struct DiceEntityMap(pub HashMap<DiceID, Entity>);

#[derive(Resource, Default, Clone)]
pub struct Rows {
  pub team1: Vec<DiceID>,
  pub team2: Vec<DiceID>,
}

fn spawn_dices(
  dice_data: Res<DiceData>,
  mut commands: Commands,
  mut dice_spawn_event: EventWriter<SpawnDices>,
  mut images: ResMut<Assets<Image>>,
) {
  assert!(dice_data.team1.len() <= MAX_DICE_COUNT);
  assert!(dice_data.team2.len() <= MAX_DICE_COUNT);

  for i in 0..dice_data.team1.len() {
    let dice_id = DiceID { team_id: 0, dice_id: i };
    let dice = Dice::build(dice_data.team1[i].clone(), dice_id, &mut images);
    for (face, handle) in dice.faces() {
      commands.run_system_cached_with(spawn_dice_camera, handle.clone());
      commands.run_system_cached_with(update_dice_face, (face, handle));
    }
    commands.run_system_cached_with(spawn_dice, dice);
  }

  for i in 0..dice_data.team2.len() {
    let dice_id = DiceID { team_id: 1, dice_id: i };
    let dice = Dice::build(dice_data.team2[i].clone(), dice_id, &mut images);
    for (face, handle) in dice.faces() {
      commands.run_system_cached_with(spawn_dice_camera, handle.clone());
      commands.run_system_cached_with(update_dice_face, (face, handle));
    }
    commands.run_system_cached_with(spawn_dice, dice);
  }
  dice_spawn_event.write(SpawnDices);
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

async fn despawn_dead_dice(event: Arc<Mutex<DiceDied>>) -> Result<(), AccessError> {
  let dice_id = event.get().dice_id;
  let entity = get_dice_entity(dice_id).await?;
  AsyncWorld.entity(entity).despawn();
  Ok(())
}

pub async fn _fetch_current_face(
  entity: Entity,
) -> Result<usize, AccessError> {
  fetch!(entity, Transform).get(|transform| { get_face_id(transform.rotation) })
}
