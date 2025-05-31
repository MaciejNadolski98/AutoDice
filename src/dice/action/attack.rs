use futures_::future::join3;
use bevy_defer::AccessError;

use super::helpers::*;
use crate::dice::{animation::spin_dice, DiceID};
use super::interaction::dice::damage;

pub async fn attack(pips_count: u32, dice_id: DiceID) -> Result<(), AccessError> {
  if let Some(target_id) = select_enemy(dice_id).await? {
    join3(
      delayed(0.25, damage(target_id, pips_count)),
      spin_dice(dice_id, 0.5), 
      spin_dice(target_id, 0.5)
    ).await.try_all()?;
  }
  Ok(())
}
