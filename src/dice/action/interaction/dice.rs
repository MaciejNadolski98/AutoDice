use bevy::prelude::*;
use bevy_defer::{fetch, AccessError, AsyncAccess, AsyncWorld};

use crate::dice::status::Status;
use crate::dice::{animation::get_dice_entity, events::DiceDied, Dice, DiceID};
use crate::utils::*;
use crate::battle::SpawnFloatingText;

pub async fn damage(
  dice_id: DiceID,
  damage: u32,
) -> Result<(), AccessError> {
  let mut died = false;
  let entity = get_dice_entity(dice_id).await?;
  let position = fetch!(entity, Transform).get(|t| t.translation)?;
  AsyncWorld.send_event(SpawnFloatingText::new(format!("-{}", damage), position))?;
  fetch!(entity, Dice).get_mut(|dice| {
    let new_hp = dice.current_hp().saturating_sub(damage);
    dice.set_current_hp(new_hp);
    if new_hp == 0 {
      died = true;
    }
  })?;
  if died {
    AsyncWorld.trigger_event(DiceDied::new(DiceDied { dice_id })).await?;
  }
  Ok(())
}

#[allow(dead_code)]
pub async fn heal(
  dice_id: DiceID,
  heal_amount: u32,
) -> Result<(), AccessError> {
  let entity = get_dice_entity(dice_id).await?;
  fetch!(entity, Dice).get_mut(|dice| {
    let new_hp = (dice.current_hp() + heal_amount).min(dice.max_hp());
    dice.set_current_hp(new_hp);
  })?;
  Ok(())
}

pub async fn apply_status<S: Status>(
  dice_id: DiceID,
  status: S,
) -> Result<(), AccessError> {
  let entity = get_dice_entity(dice_id).await?;
  let new_status = if let Ok(current_status) = fetch!(entity, S).get(|status| *status) {
    current_status.combine(status)
  } else {
    status
  };
  AsyncWorld
    .entity(entity)
    .insert(new_status)?;
  Ok(())
}
