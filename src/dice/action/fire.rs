use bevy::prelude::*;
use futures_::future::join3;
use bevy_defer::AccessError;

use super::{helpers::*, interaction::dice::apply_status};
use crate::dice::{animation::spin_dice, status::Burning, DiceID};

pub async fn fire(pips: u32, dice_id: DiceID) -> Result<(), AccessError> {
  if pips == 0 {
    return Ok(())
  }
  if let Some(target_id) = select_enemy(dice_id).await? {
    join3(
      delayed(0.25, apply_status(target_id, Burning { intensity: pips })),
      spin_dice(dice_id, 0.5), 
      spin_dice(target_id, 0.5)
    ).await.try_all()?;
  }
  Ok(())
}
