use bevy::prelude::*;

use crate::{camera::BattleCamera, constants::{FLOATING_TEXT_DURATION, FLOATING_TEXT_FONT_SIZE, FLOATING_TEXT_SPEED}};

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
  position: Vec2,
  timer: Timer,
}

impl FloatingText {
  pub fn new(position: Vec2) -> Self {
    Self {
      position,
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
      Name::new("Floating text"),
      Node {
        display: Display::None,
        ..default()
      },
      Text(event.text.clone()),
      TextFont {
        font_size: FLOATING_TEXT_FONT_SIZE,
        ..default()
      },
      TextColor(event.color),
      FloatingText::new(event.position.truncate()),
    ));
  }
}

fn floating_text_system(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(Entity, &mut Node, &ComputedNode, &mut TextColor, &mut FloatingText)>,
  camera: Single<(&Camera, &GlobalTransform), With<BattleCamera>>,
) {
  let (camera, camera_transform) = *camera;
  for (entity, mut node, &ComputedNode { size, inverse_scale_factor, .. }, mut color, mut floating_text) in query.iter_mut() {
    floating_text.timer.tick(time.delta());
    let Ok(viewport_position) = camera.world_to_viewport(camera_transform, floating_text.position.extend(0.0)) else { continue };
    let elapsed = floating_text.timer.elapsed().as_secs_f32();
    node.display = Display::Flex;
    node.top = Val::Px(viewport_position.y - elapsed * inverse_scale_factor * FLOATING_TEXT_SPEED);
    node.left = Val::Px(viewport_position.x - 0.5 * size.y * inverse_scale_factor);
    let mut rgba = color.0.to_linear();
    rgba.alpha = 1.0 - elapsed / FLOATING_TEXT_DURATION;
    *color = Color::LinearRgba(rgba).into();

    if floating_text.timer.finished() {
      commands.entity(entity).despawn();
    }
  }
}