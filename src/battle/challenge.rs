use crate::{constants::DICE_COUNT, dice::DiceTemplateBuilder};

pub struct Challenge(pub [DiceTemplateBuilder; DICE_COUNT]);

impl Challenge {
  pub fn new(level: u32) -> Self {
    Self([
      DiceTemplateBuilder::berserker(level),
      DiceTemplateBuilder::paladin(level),
      DiceTemplateBuilder::cleric(level),
      DiceTemplateBuilder::mage(level),
      DiceTemplateBuilder::rogue(level),
    ])
  }
}
