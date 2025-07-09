use std::marker::PhantomData;

use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use super::events::SpawnDices;
use super::dice_instance::Dice;

use crate::{camera::BattleCamera, constants::dice_info_bar::*, dice::{dice_instance::Health, status::{Burning, Regeneration, Status}}, states::GameState};

pub struct DiceInfoBarPlugin;

impl Plugin for DiceInfoBarPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, spawn_dice_info_bars.run_if(on_event::<SpawnDices>))
      .add_systems(PostUpdate, update_dice_info_bar_positions.run_if(in_state(GameState::Battle)))
      .add_systems(Update, update_health_bar_indicator)
      .add_systems(Update, (
        update_status_intensity::<Burning>,
        update_status_intensity::<Regeneration>,
      ));
  }
}

#[derive(Component)]
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

pub struct HealthBar;

impl HealthBar {
  pub fn spawn(commands: &mut RelatedSpawnerCommands<ChildOf>, health_entity: Entity) {
    commands.spawn((
      Name::new("Health bar"),
      Node {
        width: Val::Px(HEALTH_BAR_WIDTH),
        height: Val::Px(HEALTH_BAR_HEIGHT),
        ..default()
      },
      Visibility::default()
    )).with_children(|commands|{
      commands.spawn((
        Name::new("Gray background"),
        Node {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          justify_content: JustifyContent::Stretch,
          ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        HealthIndicatorOf { dice: health_entity },
      ));
    });
  }
}

fn spawn_dice_info_bars(
  mut commands: Commands,
  dices: Query<Entity, With<Dice>>,
) {
  for dice_entity in &dices {
    commands
      .spawn((
        Name::new("Dice info bar"),
        Node {
          position_type: PositionType::Absolute,
          flex_direction: FlexDirection::Column,
          ..default()
        },
        DiceInfoBar,
        DiceInfoOf {
          dice: dice_entity
        },
      ))
      .with_children(|commands| {
        HealthBar::spawn(commands, dice_entity);

        commands.spawn((
          Name::new("Status bar"),
          StatusBarOf { dice: dice_entity },
          Node {
            width: Val::Percent(100.0),
            ..default()
          },
          Visibility::default()
        ));
      });
  }
}

fn update_dice_info_bar_positions(
  mut dice_info_bars: Query<(&DiceInfoOf, &mut Node, &ComputedNode), With<DiceInfoBar>>,
  dice_transforms: Query<&Transform, (With<Dice>, Without<DiceInfoBar>)>,
  camera: Single<(&Camera, &GlobalTransform), With<BattleCamera>>,
) {
  let (camera, camera_transform) = *camera;
  for (dice_info_of, mut bar_node, ComputedNode { size, inverse_scale_factor, .. }) in dice_info_bars.iter_mut() {
    let Ok(dice_transform) = dice_transforms.get(dice_info_of.dice) else { continue };
    let Ok(viewport_position) = camera.world_to_viewport(camera_transform, dice_transform.translation) else { continue };
    bar_node.left = Val::Px(viewport_position.x - (size.x / 2.0) * inverse_scale_factor);
    bar_node.top = Val::Px(viewport_position.y + BAR_DISPLACEMENT * inverse_scale_factor);
  }
}

fn update_health_bar_indicator(
  dices: Query<(&Health, &HealthIndicator), Changed<Health>>,
  children: Query<&Children>,
  mut commands: Commands,
) {
  for (health, &HealthIndicator { indicator }) in dices {
    let visible = |index: usize| {
      if (index as u32) < health.current {
        Visibility::Visible
      } else {
        Visibility::Hidden
      }
    };

    let children = children.get(indicator).map(|x| x.into_iter()).unwrap_or_default().collect::<Vec<_>>();
    for (index, &&child) in children.iter().enumerate() {
      commands.entity(child).insert((visible(index), health_color(health.current as f32 / health.max as f32)));
    }

    if (children.len() as u32) < health.max {
      commands.entity(indicator).with_children(|commands| {
        for index in children.len()..(health.max as usize) {
          commands.spawn((
            Name::new("Health segment"),
            Node {
              height: Val::Percent(100.0),
              flex_grow: 1.0,
              margin: UiRect::all(Val::Px(HEALTH_BAR_MARGIN)),
              ..default()
            },
            visible(index),
            BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
          ));
        }
      });
    } else {
      for &&child in children.iter().skip(health.max as usize) {
        commands.entity(child).despawn();
      }
    }
  }
}

fn health_color(percentage: f32) -> BackgroundColor {
  BackgroundColor(
    if percentage > 0.5 {
      let t = (percentage - 0.5) * 2.0;
      Color::srgb(1.0 - t, 1.0, 0.0)
    } else {
      let t = percentage * 2.0;
      Color::srgb(1.0, t, 0.0)
    }
  )
}

fn update_status_intensity<S: Status>(
  statuses: Query<(&S, &StatusIntensity<S>), Changed<S>>,
  mut texts: Query<&mut Text>,
) {
  for (status, intensity) in statuses {
    if let Ok(mut text) = texts.get_mut(intensity.text) {
      if let Some(intensity) = status.intensity() {
        text.0 = format!("{intensity}");
      }
    }
  }
}
