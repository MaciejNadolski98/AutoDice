use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use rand::{seq::SliceRandom, thread_rng};

use crate::dice::{Action, Face, FacePrototype, Gridable};

#[derive(Component, Clone)]
pub struct Tile {
  grid: Vec<(i16, i16)>,
}

impl Gridable for Tile {
  fn grid(&self) -> Vec<(i16, i16)> {
    self.grid.clone()
  }
}

impl Tile {
  pub fn spawn(mut images: &mut Assets<Image>, commands: &mut RelatedSpawnerCommands<ChildOf>) {
    let grid = build_tile_layout();
    let faces_count = grid.len();
    commands.spawn(Self { grid })
      .with_children(|commands|{
        for _ in 0..faces_count {
          let prototype = random_face();
          commands.spawn(Face::from_prototype(prototype, &mut images));
        }
    });
  }
}

fn random_face() -> FacePrototype {
  *[
    FacePrototype::new(Action::Attack, Some(2)),
    FacePrototype::new(Action::Attack, Some(1)),
    FacePrototype::new(Action::Defend, None),
    FacePrototype::new(Action::Fire, Some(1)),
    FacePrototype::new(Action::Fire, Some(2)),
    FacePrototype::new(Action::Regenerate, Some(2)),
    FacePrototype::new(Action::Regenerate, Some(1)),
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
