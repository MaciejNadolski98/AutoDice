use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::dice::{background::FaceBackground, dice_template::{face_prototypes::{ATTACK_STRONG, ATTACK_STRONG_CRUEL, ATTACK_WEAK, DEFEND, FIRE_STRONG, FIRE_WEAK, REGEN_STRONG, REGEN_WEAK}, face_sets::*}, face::Face, Gridable};

use super::action::Action;

pub struct DiceTemplatePlugin;

impl Plugin for DiceTemplatePlugin {
  fn build(&self, _app: &mut App) {
  }
}

#[derive(Component, Clone)]
pub struct DiceTemplate {
  pub hp: u32,
}

impl Gridable for DiceTemplate {
  fn grid(&self) -> Vec<(i16, i16)> {
    vec![(2, 1), (1, 2), (3, 2), (2, 2), (2, 3), (2, 4)]
  }
}

#[allow(dead_code)]
pub enum FaceId {
  Top = 0,
  Left = 1,
  Right = 2,
  Middle = 3,
  NearBottom = 4,
  FarBottom = 5,
}

#[derive(Default, Clone, Copy)]
pub struct DiceTemplateBuilder {
  faces: Option<[FacePrototype; 6]>,
  hp: Option<u32>,
}

impl DiceTemplateBuilder {
  pub fn spawn(self, commands: &mut RelatedSpawnerCommands<ChildOf>, mut images: &mut Assets<Image>) {
    assert!(self.hp != None);
    assert!(self.faces != None);

    let template = DiceTemplate { hp: self.hp.unwrap() };
    commands
      .spawn(template)
      .with_children(|commands| {
        self.faces.clone().unwrap().map(|face|{
          commands.spawn(Face::from_prototype(face, &mut images));
        });
      });
  }

  pub fn with_hp(mut self, hp: u32) -> Self {
    self.hp = Some(hp);
    self
  }

  pub fn with_face_set(mut self, faces: [FacePrototype; 6]) -> Self {
    self.faces = Some(faces);
    self
  }

  pub fn with_face(mut self, face_id: FaceId, face: FacePrototype) -> Self {
    self.faces.as_mut().unwrap()[face_id as usize] = face;
    self
  }

  pub fn berserker(level: u32) -> Self {
    assert!(1 <= level && level <= 4);
    let mut ret = Self::default()
      .with_hp(5 + level)
      .with_face_set(BERSERKER);
    if level == 1 { return ret }
    ret = ret.with_face(FaceId::Middle, ATTACK_STRONG);
    if level == 2 { return ret }
    ret = ret
      .with_face(FaceId::Top, DEFEND);
    if level == 3 { return ret }
    ret
      .with_face(FaceId::Left, ATTACK_STRONG_CRUEL.with_pips(3))
      .with_face(FaceId::Right, ATTACK_STRONG_CRUEL.with_pips(3))
  }

  pub fn paladin(level: u32) -> Self {
    assert!(1 <= level && level <= 4);
    let mut ret = Self::default()
      .with_hp(4 + 2 * level)
      .with_face_set(PALADIN);
    if level == 1 { return ret }
    ret = ret.with_face(FaceId::Left, ATTACK_STRONG);
    if level == 2 { return ret }
    ret = ret
      .with_face(FaceId::Top, ATTACK_WEAK)
      .with_face(FaceId::NearBottom, ATTACK_WEAK);
    if level == 3 { return ret }
    ret
      .with_face(FaceId::Left, ATTACK_STRONG_CRUEL.with_pips(3))
  }

  pub fn mage(level: u32) -> Self {
    assert!(1 <= level && level <= 4);
    let mut ret = Self::default()
      .with_hp(2 + level)
      .with_face_set(MAGE);
    if level == 1 { return ret }
    ret = ret.with_face(FaceId::NearBottom, FIRE_WEAK);
    if level == 2 { return ret }
    ret = ret
      .with_face(FaceId::Top, FIRE_WEAK);
    if level == 3 { return ret }
    ret
      .with_face(FaceId::Left, FIRE_STRONG.with_pips(3))
      .with_face(FaceId::Right, FIRE_STRONG.with_pips(3))
  }

  pub fn cleric(level: u32) -> Self {
    assert!(1 <= level && level <= 4);
    let mut ret = Self::default()
      .with_hp(3 + 2 * level)
      .with_face_set(CLERIC);
    if level == 1 { return ret }
    ret = ret.with_face(FaceId::Middle, REGEN_WEAK);
    if level == 2 { return ret }
    ret = ret.with_face(FaceId::Top, REGEN_WEAK);
    if level == 3 { return ret }
    ret
      .with_face(FaceId::Left, REGEN_STRONG.with_pips(3))
      .with_face(FaceId::Right, REGEN_STRONG.with_pips(3))
  }

  pub fn rogue(level: u32) -> Self {
    assert!(1 <= level && level <= 4);
    let mut ret = Self::default()
      .with_hp(3 + level)
      .with_face_set(ROGUE);
    if level == 1 { return ret }
    ret = ret.with_face(FaceId::Middle, ATTACK_WEAK);
    if level == 2 { return ret }
    ret = ret.with_face(FaceId::Top, ATTACK_WEAK);
    if level == 3 { return ret }
    ret
      .with_face(FaceId::NearBottom, ATTACK_WEAK)
      .with_face(FaceId::Left, ATTACK_STRONG_CRUEL)
  }
}

#[derive(PartialEq, Eq, Default, Clone, Copy, Hash, Debug)]
pub struct FacePrototype {
  pub action: Action,
  pub pips: Option<u32>,
  pub background: FaceBackground,
}

impl FacePrototype {
  pub const fn new(action: Action, pips: Option<u32>) -> Self {
    Self { action, pips, background: FaceBackground::Empty }
  }

  const fn with_pips(mut self, pips: u32) -> Self {
    self.pips = Some(pips);
    self
  }

  const fn with_background(mut self, background: FaceBackground) -> Self {
    self.background = background;
    self
  }
}

pub mod face_prototypes {
  use crate::dice::{background::FaceBackground, dice_template::FacePrototype, Action};

  pub const EMPTY: FacePrototype = FacePrototype::new(Action::Empty, None);
  pub const ATTACK_WEAK: FacePrototype = FacePrototype::new(Action::Attack, Some(1));
  pub const ATTACK_DOUBLE: FacePrototype = FacePrototype::new(Action::Attack, Some(1))
    .with_background(FaceBackground::Double);
  pub const ATTACK_STRONG: FacePrototype = FacePrototype::new(Action::Attack, Some(2));
  pub const ATTACK_STRONG_CRUEL: FacePrototype = FacePrototype::new(Action::Attack, Some(2))
    .with_background(FaceBackground::Cruel);
  pub const DEFEND: FacePrototype = FacePrototype::new(Action::Defend, None);
  pub const REGEN_WEAK: FacePrototype = FacePrototype::new(Action::Regenerate, Some(1));
  pub const REGEN_STRONG: FacePrototype = FacePrototype::new(Action::Regenerate, Some(2));
  pub const FIRE_WEAK: FacePrototype = FacePrototype::new(Action::Fire, Some(1));
  pub const FIRE_STRONG: FacePrototype = FacePrototype::new(Action::Fire, Some(2));
}

mod face_sets {
  use crate::dice::{dice_template::FacePrototype};
  use super::face_prototypes::*;

  pub const BERSERKER: [FacePrototype; 6] = [
    EMPTY,
    ATTACK_STRONG_CRUEL,
    ATTACK_STRONG,
    EMPTY,
    EMPTY,
    EMPTY,
  ];

  pub const PALADIN: [FacePrototype; 6] = [
    EMPTY,
    ATTACK_WEAK,
    DEFEND,
    REGEN_WEAK,
    EMPTY,
    EMPTY,
  ];

  pub const MAGE: [FacePrototype; 6] = [
    EMPTY,
    FIRE_STRONG,
    FIRE_STRONG,
    REGEN_WEAK,
    EMPTY,
    EMPTY,
  ];

  pub const CLERIC: [FacePrototype; 6] = [
    EMPTY,
    DEFEND,
    REGEN_STRONG,
    EMPTY,
    EMPTY,
    EMPTY,
  ];

  pub const ROGUE: [FacePrototype; 6] = [
    EMPTY,
    ATTACK_DOUBLE,
    ATTACK_DOUBLE,
    EMPTY,
    EMPTY,
    EMPTY,
  ];
}
