use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use rand::{seq::SliceRandom, thread_rng};

use crate::dice::{Action, Face, Gridable};

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
          let (action, pips_count) = random_face();
          commands.spawn(Face::new(action, pips_count, &mut images));
        }
    });
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
