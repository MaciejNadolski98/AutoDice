use std::{fmt::Debug, sync::Arc};

use bevy::prelude::*;
use bevy_defer::{AccessError, AsyncAccess, AsyncWorld};

use crate::utils::*;

use super::{Dice, DiceID};

mod burning;
mod plugin;
mod double;

pub use burning::Burning;
pub use double::Double;
pub use plugin::StatusPlugin;

pub trait Status: Component<Mutability = bevy::ecs::component::Mutable> + Clone + Copy {
  type TriggerEvent: Event + Clone + Copy + Debug;

  fn trigger_condition(&self, _dice: &Dice, _event: Self::TriggerEvent) -> bool {
    true
  }

  async fn resolve_status(&self, dice_id: DiceID, event: Self::TriggerEvent) -> Result<(), AccessError>;

  async fn update_event(&self, _dice_id: DiceID, event: Self::TriggerEvent) -> Result<Self::TriggerEvent, AccessError> {
    Ok(event)
  }
  
  fn update(&mut self) -> bool;

  fn combine(self, other: Self) -> Self;
}

trait Registrable {
  fn register(app: &mut App);
}

trait TriggersToEvent {
  type EventType: Event + Clone + Copy + Debug;

  fn register_listener(listener: DynAsyncFunction<Self::EventType>, app: &mut App) {
    app.register_dyn_listener(listener);
  }

  fn get_event_listener() -> DynAsyncFunction<Self::EventType>;
}

impl<T: TriggersToEvent> Registrable for T {
  fn register(app: &mut App) {
    Self::register_listener(Self::get_event_listener(), app);
  }
}

impl<S> TriggersToEvent for S
where
  S: Status,
{
  type EventType = <S as Status>::TriggerEvent;

  fn get_event_listener() -> DynAsyncFunction<Self::EventType> {
    Arc::new(move |event| {
      Box::pin(async move {
        let mut dice_ids = Vec::new();
        AsyncWorld
          .query::<(Entity, &Dice, &S)>()
          .for_each(|(entity, dice, status)| {
            if status.trigger_condition(dice, event.get()) {
              dice_ids.push((entity, dice.id(), *status));
            }
          });
        let statuses = AsyncWorld
          .query::<&mut S>();
        let mut new_event = event.get();

        for (entity, dice_id, status) in dice_ids {
          status.resolve_status(dice_id, event.get()).await?;
          if Ok(true) == statuses.entity(entity).get_mut(|mut status| status.update()) {
            AsyncWorld.entity(entity).component::<S>().remove();
          }
          new_event = status.update_event(dice_id, new_event).await?;
        }
        event.mutate(|_| new_event);
        Ok(())
      })
    })
  }
}

trait RegisterRegistrable {
  fn register<R: Registrable>(&mut self) -> &mut Self;
}

impl RegisterRegistrable for App {
  fn register<R: Registrable>(&mut self) -> &mut Self {
    R::register(self);
    self
  }
}
