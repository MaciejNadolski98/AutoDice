use std::marker::PhantomData;

use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::{dice::{synergy::{RegisterSynergy, Synergy, SynergyTooltip, TeamSynergy}, Face}, manage::plugin::{EnemyTeam, MyTeam}, utils::tooltip::update_tooltips};

use super::Fiery;

pub struct SynergyPlugin;

impl Plugin for SynergyPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<TeamSynergy<Fiery>>()
      .register::<Fiery>()
      .add_systems(Update, (update_team_synergy::<Fiery>, update_synergy_display::<Fiery>, update_tooltips::<SynergyTooltip<Fiery>>));
  }
}

pub fn spawn_synergy_displays(
  commands: &mut RelatedSpawnerCommands<ChildOf>,
) {
  commands.spawn((
    Name::new("Synergy display: Fiery"),
    SynergyDisplay::<Fiery>::new(0),
  ));
}

#[derive(Component)]
#[require(SynergyTooltip::<S>::new(), Text)]
pub struct SynergyDisplay<S: Synergy> {
  team_id: usize,
  _phantom: PhantomData<S>,
}

impl<S: Synergy> SynergyDisplay<S> {
  pub fn new(team_id: usize) -> Self {
    SynergyDisplay {
      team_id,
      _phantom: PhantomData,
    }
  }
}

fn update_synergy_display<S: Synergy>(
  synergies: Res<TeamSynergy<S>>,
  displays: Query<(&mut Text, &mut Node, &SynergyDisplay<S>)>,
) {
  if !synergies.is_changed() { return; }

  for (mut text, mut node, &SynergyDisplay { team_id, .. }) in displays {
    match synergies.synergies[team_id] {
      Some(synergy) => {
        node.display = Display::Flex;
        text.0 = format!("{}: {}", S::name(), synergy.intensity());
      },
      None => {
        node.display = Display::None;
      }
    }
  }
}

fn update_team_synergy<S: Synergy>(
  my_team: Single<Entity, With<MyTeam>>,
  enemy_team: Single<Entity, With<EnemyTeam>>,
  children: Query<&Children>,
  faces: Query<&Face>,
  mut synergies: ResMut<TeamSynergy<S>>,
) {
  for team_id in 0..2 {
    let mut intensity = 0;
    let team_entity = if team_id == 0 { *my_team } else { *enemy_team };
    for &template_entity in children.get(team_entity).unwrap() {
      for &face_entity in children.get(template_entity).unwrap() {
        let face = faces.get(face_entity).unwrap().prototype;
        intensity += S::read_face(face);
      }
    }
    synergies.synergies[team_id] = S::new(intensity, team_id);
  }
}
