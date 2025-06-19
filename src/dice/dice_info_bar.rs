use std::marker::PhantomData;

use bevy::prelude::*;

use super::events::SpawnDices;
use super::dice_instance::Dice;

use crate::{constants::{dice_info_bar::*, BATTLE_OVERLAY_LAYER}, dice::status::{Burning, Regeneration, Status}, states::GameState};

pub struct DiceInfoBarPlugin;

impl Plugin for DiceInfoBarPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, spawn_dice_info_bars.run_if(on_event::<SpawnDices>))
      .add_systems(PostUpdate, (update_dice_info_bar_positions, update_health_bar_indicator).chain().run_if(in_state(GameState::Battle)))
      .add_systems(Update, update_status_icon_positions)
      .add_systems(Update, (
        update_status_intensity::<Burning>,
        update_status_intensity::<Regeneration>,
      ));
  }
}

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct DiceInfoBar;

#[derive(Component)]
#[relationship(relationship_target = DiceInfo)]
pub struct DiceInfoOf {
  dice: Entity,
}

#[derive(Component)]
#[relationship_target(relationship = DiceInfoOf, linked_spawn)]
pub struct DiceInfo {
  dice_info: Entity,
}

#[derive(Component)]
#[relationship(relationship_target = StatusBar)]
pub struct StatusBarOf {
  dice: Entity,
}

#[derive(Component)]
#[relationship_target(relationship = StatusBarOf)]
pub struct StatusBar {
  bar: Entity,
}

impl StatusBar {
  pub fn bar(&self) -> Entity {
    self.bar
  }
}

#[derive(Component)]
#[relationship(relationship_target = StatusIcon<S>)]
pub struct StatusIconOf<S: Status> {
  #[relationship]
  dice: Entity,
  _marker: PhantomData<S>,
}

impl<S: Status> StatusIconOf<S> {
  pub fn new(dice: Entity) -> Self {
    Self { dice, _marker: PhantomData::<S> }
  }
}

#[derive(Component)]
#[relationship_target(relationship = StatusIconOf<S>)]
pub struct StatusIcon<S: Status> {
  #[relationship]
  icon: Entity,
  _marker: PhantomData<S>,
}

impl<S: Status> StatusIcon<S> {
  pub fn icon(&self) -> Entity {
    self.icon
  }
}

#[derive(Component)]
#[relationship(relationship_target = StatusIntensity<S>)]
pub struct StatusIntensityOf<S: Status> {
  #[relationship]
  dice: Entity,
  _marker: PhantomData<S>,
}

impl<S: Status> StatusIntensityOf<S> {
  pub fn new(dice: Entity) -> Self {
    Self {
      dice,
      _marker: PhantomData::<S>,
    }
  }
}

#[derive(Component)]
#[relationship_target(relationship = StatusIntensityOf<S>, linked_spawn)]
pub struct StatusIntensity<S: Status> {
  #[relationship]
  text: Entity,
  _marker: PhantomData<S>,
}

#[derive(Component)]
#[relationship(relationship_target = HealthIndicator)]
pub struct HealthIndicatorOf {
  dice: Entity,
}

#[derive(Component)]
#[relationship_target(relationship = HealthIndicatorOf)]
pub struct HealthIndicator {
  indicator: Entity,
}

fn spawn_dice_info_bars(
  mut commands: Commands,
  dices: Query<Entity, With<Dice>>,
) {
  for dice_entity in &dices {
    commands
      .spawn((
        Name::new("Dice info bar"),
        DiceInfoBar,
        DiceInfoOf {
          dice: dice_entity
        },
      ))
      .with_children(|commands| {
        commands.spawn((Name::new("Health bar"), Transform::from_translation(HEALTH_BAR_POSITION), Visibility::default())).with_children(|commands|{
          commands.spawn((
            Name::new("Colored bar"),
            Sprite::from_color(Color::srgb(0.5, 0.5, 0.5), Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
            BATTLE_OVERLAY_LAYER,
          ));

          commands.spawn((
            Name::new("Gray background"),
            Sprite::from_color(Color::srgb(0.0, 1.0, 0.0), Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
            BATTLE_OVERLAY_LAYER,
            HealthIndicatorOf { dice: dice_entity },
          ));
        });

        commands.spawn((Name::new("Status bar"), StatusBarOf { dice: dice_entity }, Transform::from_translation(STATUS_BAR_POSITION), Visibility::default()));
      });
  }
}

fn update_dice_info_bar_positions(
  mut dice_info_bars: Query<(&DiceInfoOf, &mut Transform), With<DiceInfoBar>>,
  dice_transforms: Query<&Transform, (With<Dice>, Without<DiceInfoBar>)>,
) {
  for (dice_info_of, mut bar_transform) in dice_info_bars.iter_mut() {
    if let Ok(dice_transform) = dice_transforms.get(dice_info_of.dice) {
      bar_transform.translation = dice_transform.translation - Vec3::Z * dice_transform.translation.z;
    }
  }
}

fn update_health_bar_indicator(
  mut health_bars: Query<(&HealthIndicatorOf, &mut Sprite, &mut Transform)>,
  dices: Query<&Dice, Changed<Dice>>,
) {
  for (health_indicator_of, mut sprite, mut transform) in health_bars.iter_mut() {
    if let Ok(dice) = dices.get(health_indicator_of.dice) {
      let width = (dice.current_hp() as f32 / dice.max_hp() as f32) * HEALTH_BAR_WIDTH;
      sprite.custom_size = Some(Vec2::new(width, HEALTH_BAR_HEIGHT));
      transform.translation.x = -(HEALTH_BAR_WIDTH - width) / 2.0;

      let percentage = dice.current_hp() as f32 / dice.max_hp() as f32;
      sprite.color = health_color(percentage);
    }
  }
}

fn health_color(percentage: f32) -> Color {
  if percentage > 0.5 {
    let t = (percentage - 0.5) * 2.0;
    Color::srgb(1.0 - t, 1.0, 0.0)
  } else {
    let t = percentage * 2.0;
    Color::srgb(1.0, t, 0.0)
  }
}

fn update_status_intensity<S: Status>(
  statuses: Query<(&S, &StatusIntensity<S>), Changed<S>>,
  mut texts: Query<&mut Text2d>,
) {
  for (status, intensity) in statuses {
    if let Ok(mut text) = texts.get_mut(intensity.text) {
      if let Some(intensity) = status.intensity() {
        text.0 = format!("{}", intensity);
      }
    }
  }
}

fn update_status_icon_positions(
  status_bar: Query<&Children, (Changed<Children>, With<StatusBarOf>)>,
  mut transforms: Query<&mut Transform>,
) {
  for children in status_bar {
    let children_count = children.len();
    // Calculate the starting x so that icons are centered in the status bar
    let total_width = (children_count as f32) * STATUS_ICON_SIZE.x + ((children_count as f32 - 1.0) * STATUS_MARGIN);
    let starting_x = -total_width / 2.0 + STATUS_ICON_SIZE.x / 2.0;

    for (i, child) in children.iter().enumerate() {
      let mut transform = transforms.get_mut(child).unwrap();
      let x = starting_x + i as f32 * (STATUS_ICON_SIZE.x + STATUS_MARGIN);
      transform.translation = Vec3::new(x, 0.0, 0.0);
    }
  }
}
