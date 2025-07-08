use bevy::ecs::query::QueryData;
use bevy_defer::{AccessError, AsyncWorld};

use crate::dice::action::ResolutionContext;
use crate::dice::background::FaceBackground;
use crate::dice::dice_instance::Health;
use crate::dice::{Dice, DiceID};
use rand::seq::SliceRandom;
use rand::thread_rng;


pub async fn select_enemy(context: ResolutionContext) -> Result<Option<DiceID>, AccessError> {
  let filter = |dice: &Dice| { dice.id().team_id != context.dice_id.team_id };

  if context.face.background == FaceBackground::Cruel {
    return select_random_best::<i32, &Dice, &Health>(
      filter,
      |health: &Health| { -(health.current as i32) },
    ).await;
  }

  select_random(filter).await
}

pub async fn select_ally(context: ResolutionContext) -> Result<Option<DiceID>, AccessError> {
  let filter = |dice: &Dice| {
    dice.id().team_id == context.dice_id.team_id
    && dice.id() != context.dice_id
  };

  if context.face.background == FaceBackground::Cruel {
    return select_random_best::<i32, &Dice, &Health>(
      filter,
      |health: &Health| { -(health.current as i32) },
    ).await;
  }

  select_random(filter).await
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

pub async fn select_random_best<S, FilterData: QueryData + 'static, ScoreData: QueryData + 'static>(
  filter: impl Fn(FilterData::Item<'_>) -> bool,
  score: impl Fn(ScoreData::Item<'_>) -> S
) -> Result<Option<DiceID>, AccessError>
where S: Ord + Eq + Copy {
  let mut best_dices = Vec::new();
  let mut best_score = None;

  AsyncWorld
    .query::<(FilterData, ScoreData, &Dice)>()
    .for_each(|data| {
      let (filter_data, score_data, dice) = data;
      if filter(filter_data) {
        let current_score = score(score_data);
        if best_score.is_none() || current_score > best_score.unwrap() {
          best_score = Some(current_score);
          best_dices.clear();
          best_dices.push(dice.id());
        }
        if best_score == Some(current_score) {
          best_dices.push(dice.id());
        }
      }
    });

  Ok(best_dices.select_random())
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
      if best_score.is_none() || current_score > best_score.unwrap() {
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
