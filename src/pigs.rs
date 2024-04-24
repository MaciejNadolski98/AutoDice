use bevy::prelude::*;

use crate::Player;
use crate::Money;

pub struct PigPlugin;

impl Plugin for PigPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_pig_parent)
      .add_systems(Update, (spawn_pig, pig_lifetime))
      .register_type::<Pig>();
  }
}

#[derive(Component)]
pub struct PigParent;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Pig {
  pub lifetime: Timer,
}

fn spawn_pig(
  mut commands: Commands,
  player: Query<&mut Transform, With<Player>>,
  pig_parent: Query<Entity, With<PigParent>>,
  mut money: ResMut<Money>,
  input: Res<ButtonInput<KeyCode>>,
  asset_server: Res<AssetServer>,
) {
  if !input.just_pressed(KeyCode::Space) {
    return;
  }

  let player_transform = player.single();
  let pig_parent = pig_parent.single();

  if money.0 >= 10.0 {
    money.0 -= 10.0;
    info!("10 dollars were spent to spawn a pig");

    commands.entity(pig_parent).with_children(|commands| {
      let texture = asset_server.load("pig.png");
      commands.spawn((
        SpriteBundle {
          texture,
          transform: *player_transform,
          ..default()
        },
        Pig { lifetime: Timer::from_seconds(10.0, TimerMode::Once) },
        Name::new("Pig"),
      ));
    });
  }
}

pub fn spawn_pig_parent(
  mut commands: Commands,
) {
  commands.spawn((
    SpatialBundle::default(),
    PigParent,
    Name::new("Pig Parent"),
  ));
}

fn pig_lifetime(
  mut commands: Commands,
  mut pigs: Query<(Entity, &mut Pig)>,
  pig_parent: Query<Entity, With<PigParent>>,
  mut money: ResMut<Money>,
  time: Res<Time>,
) {
  let parent = pig_parent.single();
  for (pig_entity, mut pig) in &mut pigs {
    pig.lifetime.tick(time.delta());
    if pig.lifetime.finished() {
      info!("Pig generated 15 dollars! New money #${:?}", money.0);
      money.0 += 15.0;

      commands.entity(parent).remove_children(&[pig_entity]);
      commands.entity(pig_entity).despawn();
    }
  }
}