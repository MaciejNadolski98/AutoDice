use bevy::prelude::*;
use bevy_defer::{fetch, AccessError, AsyncAccess, AsyncWorld};

use crate::dice::events::SpawnDices;
use crate::manage::plugin::{EnemyTeam, MyTeam};
use crate::states::GameState;
use crate::utils::*;

use super::animation::get_dice_entity;
use super::dice_render::spawn_dice;
use super::dice_template::DiceTemplate;
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
pub struct Dice {
  id: DiceID,
  max_hp: u32,
  current_hp: u32,
  row_position: usize,
}

impl Dice {
  pub fn build(
    template: DiceTemplate,
    dice_id: DiceID,
  ) -> Self {
    let mut dice = Dice::default();
    dice.set_id(dice_id);
    dice.set_max_hp(template.hp);
    dice.set_current_hp(template.hp);
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
  mut commands: Commands,
  mut dice_spawn_event: EventWriter<SpawnDices>,
  my_team: Single<&Children, With<MyTeam>>,
  enemy_team: Single<&Children, With<EnemyTeam>>,
) {
  for (i, my_template) in my_team.iter().enumerate() {
    let dice_id = DiceID { team_id: 0, dice_id: i };
    commands.run_system_cached_with(spawn_dice, (dice_id, my_template));
  }

  for (i, enemy_template) in enemy_team.iter().enumerate() {
    let dice_id = DiceID { team_id: 1, dice_id: i };
    commands.run_system_cached_with(spawn_dice, (dice_id, enemy_template));
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
