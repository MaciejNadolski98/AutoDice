use bevy_defer::AccessError;

use crate::dice::DiceID;

pub async fn attack(_pips_count: u32, _dice_id: DiceID) -> Result<(), AccessError> {
  Ok(())
}
