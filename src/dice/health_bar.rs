use bevy::prelude::*;

use super::events::SpawnDices;
use super::dice_instance::Dice;

use crate::{constants::{BATTLE_OVERLAY_LAYER, DICE_SIZE, HEALTH_BAR_HEIGHT, HEALTH_BAR_WIDTH}, states::GameState};

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, spawn_health_bars.run_if(on_event::<SpawnDices>))
      .add_systems(PostUpdate, (despawn_orphan_health_bars, update_health_bar_position, update_health_bar_indicator).chain().run_if(in_state(GameState::Battle)));
  }
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct RelatedEntity {
  related_entity: Entity,
}

#[derive(Component)]
pub struct HealthIndicator;

fn spawn_health_bars(
  mut commands: Commands,
  dices: Query<Entity, With<Dice>>,
) {
  for dice_entity in &dices {
    commands
      .spawn((
        Name::new("Health bar"),
        Sprite::from_color(Color::srgb(0.5, 0.5, 0.5), Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
        BATTLE_OVERLAY_LAYER,
        RelatedEntity {
          related_entity: dice_entity,
        },
        HealthBar,
      )).with_children(|commands| {
        commands.spawn((
          Sprite::from_color(Color::srgb(0.0, 1.0, 0.0), Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
          Transform::from_xyz(0.0, 0.0, 1.0),
          BATTLE_OVERLAY_LAYER,
          RelatedEntity {
            related_entity: dice_entity,
          },
          HealthIndicator,
        ));
      });
  }
}

fn despawn_orphan_health_bars(
  mut commands: Commands,
  health_bars: Query<(Entity, &RelatedEntity), With<HealthBar>>,
  existing_dice: Query<Entity, With<Dice>>,
) {
  for (health_bar_entity, related) in health_bars.iter() {
    if existing_dice.get(related.related_entity).is_err() {
      commands.entity(health_bar_entity).despawn();
    }
  }
}

fn update_health_bar_position(
  mut health_bars: Query<(&RelatedEntity, &mut Transform), With<HealthBar>>,
  dice_transforms: Query<&Transform, (With<Dice>, Without<HealthBar>)>,
) {
  let offset_y = DICE_SIZE;

  for (related, mut bar_transform) in health_bars.iter_mut() {
    if let Ok(dice_transform) = dice_transforms.get(related.related_entity) {
      bar_transform.translation = dice_transform.translation - Vec3::new(0.0, offset_y, 0.0);
    }
  }
}

fn update_health_bar_indicator(
  mut health_bars: Query<(&RelatedEntity, &mut Sprite, &mut Transform), With<HealthIndicator>>,
  dices: Query<&Dice>,
) {
  fn health_color(percentage: f32) -> Color {
    // Green to yellow to red
    if percentage > 0.5 {
      // Green to yellow
      let t = (percentage - 0.5) * 2.0;
      Color::srgb(1.0 - t, 1.0, 0.0)
    } else {
      // Yellow to red
      let t = percentage * 2.0;
      Color::srgb(1.0, t, 0.0)
    }
  }

  for (related, mut sprite, _) in health_bars.iter_mut() {
    if let Ok(dice) = dices.get(related.related_entity) {
      let percentage = dice.current_hp() as f32 / dice.max_hp() as f32;
      sprite.color = health_color(percentage);
    }
  }

  for (related, mut sprite, mut transform) in health_bars.iter_mut() {
    if let Ok(dice) = dices.get(related.related_entity) {
      let width = (dice.current_hp() as f32 / dice.max_hp() as f32) * HEALTH_BAR_WIDTH;
      sprite.custom_size = Some(Vec2::new(width, HEALTH_BAR_HEIGHT));
      transform.translation.x = -(HEALTH_BAR_WIDTH - width) / 2.0;
    }
  }
}
