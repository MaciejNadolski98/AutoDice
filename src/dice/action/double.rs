use futures_::future::join;
use bevy_defer::AccessError;

use super::helpers::*;
use crate::dice::{action::interaction::dice::apply_status, animation::spin_dice, status::Double, DiceID};

pub async fn double(dice_id: DiceID) -> Result<(), AccessError> {
  join(
    delayed(0.25, apply_status(dice_id, Double)),
    spin_dice(dice_id, 0.5), 
  ).await.try_all()?;
  Ok(())
}
