use bevy::prelude::*;
use bevy_defer::{fetch, AccessError, AsyncAccess, AsyncWorld};

use crate::dice::dice_instance::Health;
use crate::dice::status::Status;
use crate::dice::{animation::get_dice_entity, events::DiceDied, DiceID};
use crate::utils::*;
use crate::battle::SpawnFloatingText;

pub async fn damage(
  dice_id: DiceID,
  damage: u32,
  color: Color,
) -> Result<(), AccessError> {
  let mut died = false;
  let entity = get_dice_entity(dice_id).await?;
  let position = fetch!(entity, Transform).get(|t| t.translation)?;
  AsyncWorld.send_event(SpawnFloatingText::new(format!("-{damage}"), position).with_color(color))?;
  fetch!(entity, Health).get_mut(|Health { current, .. }| {
    let new_hp = current.saturating_sub(damage);
    *current = new_hp;
    if new_hp == 0 {
      died = true;
    }
  })?;
  if died {
    AsyncWorld.trigger_event(DiceDied::wrap(DiceDied { dice_id })).await?;
  }
  Ok(())
}

pub async fn heal(
  dice_id: DiceID,
  heal_amount: u32,
) -> Result<(), AccessError> {
  let entity = get_dice_entity(dice_id).await?;
  let position = fetch!(entity, Transform).get(|t| t.translation)?;
  AsyncWorld.send_event(
    SpawnFloatingText::new(format!("+{heal_amount}"), position)
      .with_color(Color::linear_rgb(0.0, 1.0, 0.0))
  )?;
  fetch!(entity, Health).get_mut(|Health { max, current}| {
    let new_hp = (*current + heal_amount).min(*max);
    *current = new_hp;
  })?;
  Ok(())
}

pub async fn apply_status<S: Status>(
  dice_id: DiceID,
  status: S,
) -> Result<(), AccessError> {
  let entity = get_dice_entity(dice_id).await?;
  let new_status = if let Ok(current_status) = fetch!(entity, S).get(|status| *status) {
    AsyncWorld.entity(entity).remove::<S>()?;
    current_status.combine(status)
  } else {
    status
  };
  AsyncWorld
    .entity(entity)
    .insert(new_status)?;
  Ok(())
}
