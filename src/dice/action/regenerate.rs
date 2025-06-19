use bevy::prelude::*;
use futures_::future::join3;
use bevy_defer::AccessError;

use super::{helpers::*, interaction::dice::apply_status};
use crate::dice::{action::ResolutionContext, animation::spin_dice, status::Regeneration};

pub async fn regenerate(pips: u32, context: ResolutionContext) -> Result<(), AccessError> {
  if pips == 0 {
    return Ok(())
  }
  if let Some(target_id) = select_ally(context).await? {
    join3(
      delayed(0.25, apply_status(target_id, Regeneration { heal_amount: pips, duration_left: 3 })),
      spin_dice(context.dice_id, 0.5),
      spin_dice(target_id, 0.5)
    ).await.try_all()?;
  }
  Ok(())
}
