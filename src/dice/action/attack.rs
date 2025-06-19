use futures_::future::join3;
use bevy_defer::AccessError;
use bevy::prelude::*;

use super::helpers::*;
use crate::dice::{action::ResolutionContext, animation::spin_dice};
use super::interaction::dice::damage;

pub async fn attack(pips: u32, context: ResolutionContext) -> Result<(), AccessError> {
  if let Some(target_id) = select_enemy(context).await? {
    join3(
      delayed(0.25, damage(target_id, pips, Color::BLACK)),
      spin_dice(context.dice_id, 0.5),
      spin_dice(target_id, 0.5)
    ).await.try_all()?;
  }
  Ok(())
}
