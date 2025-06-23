use std::{marker::PhantomData, sync::Arc};

use bevy::prelude::*;
use bevy_defer::{AccessError, AsyncAccess, AsyncWorld};
use std::fmt::Debug;

mod fiery;
mod plugin;

pub use fiery::Fiery;
pub use plugin::{SynergyPlugin, spawn_synergy_displays};

use crate::{dice::FacePrototype, utils::{tooltip::Tooltip, ArcMutexMutable, DynAsyncFunction, RegisterListener}};

pub trait Synergy: Clone + Copy + Send + Sync + 'static {
  type TriggerEvent: Event + Clone + Copy + Debug;

  const SYNERGY_COLOR: Color;

  fn new(intensity: u32, team_id: usize) -> Option<Self>;

  fn name() -> &'static str;
  fn description() -> &'static str;

  fn trigger_condition(&self, _event: Self::TriggerEvent) -> bool {
    true
  }

  async fn resolve(&self, event: Self::TriggerEvent) -> Result<(), AccessError>;

  async fn update_event(&self, event: Self::TriggerEvent) -> Result<Self::TriggerEvent, AccessError> {
    Ok(event)
  }
  
  #[allow(unused)]
  fn update(&mut self) {}

  fn intensity(&self) -> u32;

  fn break_points(&self) -> &[u32];

  fn level(&self) -> u32 {
    for (i, &break_point) in self.break_points().iter().enumerate() {
      if self.intensity() < break_point {
        return i as u32;
      }
    }
    self.break_points().len() as u32
  }

  fn read_face(face: FacePrototype) -> u32;
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct TeamSynergy<S: Synergy> {
  synergies: [Option<S>; 2],
}

impl<S: Synergy> Default for TeamSynergy<S> {
  fn default() -> Self {
    Self {
      synergies: [None, None],
    }
  }
}

async fn get_team_synergy<S: Synergy>(team_id: usize) -> Result<Option<S>, AccessError> {
  AsyncWorld
    .resource::<TeamSynergy<S>>()
    .get(|synergies| synergies.synergies[team_id])
}

pub trait RegisterSynergy {
  fn register<S: Synergy>(&mut self) -> &mut Self;
}

impl RegisterSynergy for App {
  fn register<S: Synergy>(&mut self) -> &mut Self {
    let listener: DynAsyncFunction<S::TriggerEvent> = Arc::new(move |event| {
      Box::pin(async move {
        for team_id in 0..2 {
          if let Some(synergy) = get_team_synergy::<S>(team_id).await? {
            if synergy.trigger_condition(event.get()) {
              synergy.resolve(event.get()).await?;
            }
            let new_event = synergy.update_event(event.get()).await?;
            event.mutate(|_| new_event);
          }
        }
        Ok(())
      })
    });

    self.register_dyn_listener(listener);
    self
  }
}

#[derive(Component)]
#[require(Pickable::default())]
struct SynergyTooltip<S: Synergy> {
  _marker: PhantomData<S>,
}

impl<S: Synergy> SynergyTooltip<S> {
  pub fn new() -> Self {
    Self {
      _marker: PhantomData,
    }
  }
}

impl<S: Synergy> Tooltip for SynergyTooltip<S> {
  type UpdateData = ();
  type UpdateTrigger = ();

  fn check_update(
    &self,
    _tooltip: Entity,
    _query: &Query<Self::UpdateData, Self::UpdateTrigger>,
  ) -> bool {
    false
  }

  fn update(
    &self,
    _tooltip: Entity,
    _query: &Query<Self::UpdateData, Self::UpdateTrigger>,
  ) -> impl Bundle {(
    Name::new("Synergy Tooltip"),
    BackgroundColor(Color::WHITE),
    Outline {
      width: Val::Px(1.0),
      color: Color::BLACK,
      ..default()
    },
    Text::new(S::description()),
    TextColor(Color::BLACK),
    Visibility::Hidden,
    Pickable::IGNORE,
  )}
}
