use bevy::prelude::*;

use crate::constants::{BATTLE_OVERLAY_LAYER, FLOATING_TEXT_DURATION, FLOATING_TEXT_FONT_SIZE, FLOATING_TEXT_SPEED};

pub struct FloatingTextPlugin;

impl Plugin for FloatingTextPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<SpawnFloatingText>()
      .add_systems(Update, (spawn_floating_text, floating_text_system));
  }
}

#[derive(Event)]
pub struct SpawnFloatingText {
  pub text: String,
  pub position: Vec3,
  pub color: Color,
}

impl SpawnFloatingText {
  pub fn new(text: String, position: Vec3) -> Self {
    Self {
      text,
      position,
      color: Color::BLACK,
    }
  }

  pub fn with_color(mut self, color: Color) -> Self {
    self.color = color;
    self
  }
}

#[derive(Component)]
struct FloatingText {
  pub timer: Timer,
}

impl FloatingText {
  pub fn new() -> Self {
    Self {
      timer: Timer::from_seconds(FLOATING_TEXT_DURATION, TimerMode::Once),
    }
  }
}

fn spawn_floating_text(
  mut commands: Commands,
  mut event_reader: EventReader<SpawnFloatingText>,
) {
  for event in event_reader.read() {
    commands.spawn((
      Text2d(event.text.clone()),
      TextFont {
        font_size: FLOATING_TEXT_FONT_SIZE,
        ..default()
      },
      TextColor(event.color),
      Transform::from_translation(event.position),
      BATTLE_OVERLAY_LAYER,
      FloatingText::new(),
    ));
  }
}

fn floating_text_system(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(Entity, &mut Transform, &mut TextColor, &mut FloatingText)>,
) {
  for (entity, mut transform, mut color, mut floating_text) in query.iter_mut() {
    let delta = time.delta_secs();
    transform.translation.y += delta * FLOATING_TEXT_SPEED;
    floating_text.timer.tick(time.delta());
    let mut rgba = color.0.to_linear();
    rgba.alpha -= delta / FLOATING_TEXT_DURATION;
    *color = Color::LinearRgba(rgba).into();

    if floating_text.timer.finished() {
      commands.entity(entity).despawn();
    }
  }
}