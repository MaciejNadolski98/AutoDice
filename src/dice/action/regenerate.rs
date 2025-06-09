use bevy::prelude::*;
use futures_::future::join3;
use bevy_defer::AccessError;

use super::{helpers::*, interaction::dice::apply_status};
use crate::dice::{animation::spin_dice, status::Regeneration, DiceID};

pub async fn regenerate(pips_count: u32, dice_id: DiceID) -> Result<(), AccessError> {
  if pips_count == 0 {
    return Ok(())
  }
  if let Some(target_id) = select_ally(dice_id).await? {
    join3(
      delayed(0.25, apply_status(target_id, Regeneration { heal_amount: pips_count, duration_left: 3 })),
      spin_dice(dice_id, 0.5), 
      spin_dice(target_id, 0.5)
    ).await.try_all()?;
  }
  Ok(())
}
