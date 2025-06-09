use bevy_defer::{AccessError, AsyncWorld};

use crate::dice::{Dice, DiceID};
use rand::seq::SliceRandom;
use rand::thread_rng;


pub async fn select_enemy(dice_id: DiceID) -> Result<Option<DiceID>, AccessError> {
  select_random(|dice| {
    dice.id().team_id != dice_id.team_id
  }).await
}

#[allow(dead_code)]
pub async fn select_ally(dice_id: DiceID) -> Result<Option<DiceID>, AccessError> {
  select_random(|dice| {
    dice.id().team_id == dice_id.team_id && dice.id() != dice_id
  }).await
}

pub async fn select_random(filter: impl Fn(&Dice) -> bool) -> Result<Option<DiceID>, AccessError> {
  let mut filtered_dices = Vec::new();
  AsyncWorld
    .query::<&Dice>()
    .for_each(|dice| {
      if filter(dice) {
        filtered_dices.push(dice.id());
      }
    });
  Ok(filtered_dices.select_random())
}

#[allow(dead_code)]
pub async fn select_best<S>(score: impl Fn(&Dice) -> S) -> Result<Option<DiceID>, AccessError> 
where S: Ord + Eq + Copy {
  let mut best_dices = Vec::new();
  let mut best_score = None;

  AsyncWorld
    .query::<&Dice>()
    .for_each(|dice| {
      let current_score = score(dice);
      if best_score == None || current_score > best_score.unwrap() {
        best_score = Some(current_score);
        best_dices.clear();
        best_dices.push(dice.id());
      }
      if best_score == Some(current_score) {
        best_dices.push(dice.id());
      }
    });

  Ok(best_dices.select_random())
}

trait SelectRandom {
  fn select_random(&self) -> Option<DiceID>;
}

impl SelectRandom for Vec<DiceID> {
  fn select_random(&self) -> Option<DiceID> {
    if self.is_empty() {
      None
    } else {
      let mut rng = thread_rng();
      self.choose(&mut rng).copied()
    }
  }
}
