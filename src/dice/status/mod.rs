use std::{fmt::Debug, sync::Arc};

use bevy::{ecs::component::Mutable, prelude::*};
use bevy_defer::{AccessError, AsyncAccess, AsyncWorld};

use crate::utils::*;

use super::{Dice, DiceID};

mod burning;
mod plugin;
mod double;
mod regeneration;


pub use burning::Burning;
pub use double::Double;
pub use plugin::StatusPlugin;
pub use regeneration::Regeneration;

pub trait Status: Component<Mutability=Mutable> + Clone + Copy {
  type TriggerEvent: Event + Clone + Copy + Debug;

  const STATUS_COLOR: Color;

  fn description() -> &'static str;

  fn trigger_condition(&self, _dice: &Dice, _event: Self::TriggerEvent) -> bool {
    true
  }

  async fn resolve_status(&self, dice_id: DiceID, event: Self::TriggerEvent) -> Result<(), AccessError>;

  async fn update_event(&self, _dice_id: DiceID, event: Self::TriggerEvent) -> Result<Self::TriggerEvent, AccessError> {
    Ok(event)
  }
  
  fn update(&mut self) -> bool;

  fn combine(self, other: Self) -> Self;

  fn intensity(&self) -> Option<u32>;
}

#[macro_export]
macro_rules! impl_status_component {
  ($t:ty) => {
    use bevy::{ecs::component::{ComponentHook, Mutable, StorageType}};
    use $crate::dice::dice_info_bar::{StatusBar, StatusIconOf, StatusIcon, StatusIntensityOf};
    use $crate::constants::dice_info_bar::{STATUS_ICON_SIZE, STATUS_TEXT_SIZE, STATUS_MARGIN};
    use $crate::utils::tooltip::Tooltip;

    impl Component for $t {
      const STORAGE_TYPE: StorageType = StorageType::Table;
      type Mutability = Mutable;

      fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
          let bar = world
            .get::<StatusBar>(context.entity).unwrap().bar();
          world
            .commands()
            .entity(bar)
            .with_child((
              Name::new("Status Icon"),
              StatusIconOf::<Self>::new(context.entity),
              Node {
                width: Val::Px(STATUS_ICON_SIZE.x),
                height: Val::Px(STATUS_ICON_SIZE.y),
                margin: UiRect::all(Val::Px(STATUS_MARGIN)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
              },
              BackgroundColor(<$t>::STATUS_COLOR),
              related!(Children[(
                Text::new(""),
                TextFont {
                  font_size: STATUS_TEXT_SIZE,
                  ..default()
                },
                TextColor::BLACK,
                StatusIntensityOf::<Self>::new(context.entity),
                Pickable::IGNORE,
              )]),
              related!(Tooltip[(
                Name::new("Status Tooltip"),
                Text::new(Self::description()),
              )]),
            ));
        })
      }

      fn on_remove() -> Option<ComponentHook> {
        Some(|mut world, context| {
          let icon = world
            .get::<StatusIcon::<Self>>(context.entity).unwrap().icon();
          world
            .commands()
            .entity(icon)
            .despawn();
        })
      }
    }
  };
}

pub trait RegisterStatus {
  fn register<S: Status>(&mut self) -> &mut Self;
}

impl RegisterStatus for App {
  fn register<S>(&mut self) -> &mut Self
  where
    S: Status,
  {
    let listener: DynAsyncFunction<S::TriggerEvent> = Arc::new(move |event| {
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
    });
    self.register_dyn_listener(listener);
    self
  }
}
