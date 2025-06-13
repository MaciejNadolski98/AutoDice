use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use crate::dice::{spawn_dice_faces, Action, Face, FaceCollection, GridableFaceCollection};

#[derive(Component, Clone)]
#[component(on_add = spawn_dice_faces::<Tile>)]
pub struct Tile {
  faces: Vec<(i16, i16, Face)>,
}

impl FaceCollection for Tile {
  fn faces(&self) -> Vec<Face> {
    self.faces.clone().into_iter().map(|(_, _, face)| face).collect()
  }
}

impl GridableFaceCollection for Tile {
  fn gridded_faces(&self) -> Vec<(i16, i16, Face)> {
    self.faces.clone()
  }
}

impl Tile {
  pub fn generate(images: &mut Assets<Image>) -> Self {
    let mut faces = Vec::new();
    for (x, y) in build_tile_layout() {
      let (action, pips_count) = random_face();
      faces.push((x, y, Face::new(action, pips_count, images)));
    }
    Self {
      faces
    }
  }
}

fn random_face() -> (Action, u32) {
  *[
    (Action::Attack, 2),
    (Action::Attack, 1),
    (Action::Defend, 0),
    (Action::Fire, 1),
    (Action::Fire, 2),
    (Action::Regenerate, 2),
    (Action::Regenerate, 1),
  ].choose(&mut thread_rng()).unwrap()
}

fn build_tile_layout() -> Vec<(i16, i16)> {
  match *[
    "T1", "T2", "T3I", "T3L"
  ].choose(&mut thread_rng()).unwrap() {
    "T1" => vec![vec![(1, 1)]],
    "T2" => vec![
      vec![(1, 1), (1, 2)], 
      vec![(1, 1), (2, 1)],
    ],
    "T3I" => vec![
      vec![(1, 1), (1, 2), (1, 3)],
      vec![(1, 1), (2, 1), (3, 1)],
    ],
    "T3L" => vec![
      vec![(1, 1), (1, 2), (2, 2)],
      vec![(1, 1), (2, 1), (2, 2)],
      vec![(1, 2), (1, 1), (1, 2)],
      vec![(1, 2), (2, 2), (1, 2)],
    ],
    _ => panic!(),
  }.choose(&mut thread_rng()).unwrap().clone()
}
