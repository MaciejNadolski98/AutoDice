use std::marker::PhantomData;

use bevy::{ecs::{component::HookContext, query::{QueryData, QueryFilter}, world::DeferredWorld}, prelude::*, window::PrimaryWindow};

use crate::constants::{HEIGHT, WIDTH};

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
  fn build(&self, _app: &mut App) {
  }
}

pub trait Tooltip: Component {
  type UpdateData: QueryData;
  type UpdateTrigger: QueryFilter;

  fn check_update(
    &self,
    tooltip: Entity,
    query: &Query<Self::UpdateData, Self::UpdateTrigger>,
  ) -> bool;

  fn update(
    &self,
    tooltip: Entity,
    query: &Query<Self::UpdateData, Self::UpdateTrigger>,
  ) -> impl Bundle;
}

#[derive(Component)]
#[relationship(relationship_target = RelatedTooltip<T>)]
#[require(Pickable::default())]
pub struct TooltipOf<T: Tooltip> {
  #[relationship]
  entity: Entity,
  _marker: PhantomData<T>,
}

#[derive(Component)]
#[relationship_target(relationship = TooltipOf<T>, linked_spawn)]
#[component(on_add = add_hover_observer::<T>)]
pub struct RelatedTooltip<T: Tooltip> {
  #[relationship]
  tooltip: Entity,
  _marker: PhantomData<T>,
}

pub fn update_tooltips<T: Tooltip>(
  mut commands: Commands,
  tooltips: Query<(Entity, &T)>,
  related_tooltips: Query<&RelatedTooltip<T>>,
  query: Query<T::UpdateData, T::UpdateTrigger>,
) {
  for (entity, tooltip) in tooltips {
    if related_tooltips.get(entity).is_ok() && !tooltip.check_update(entity, &query) {
      continue;
    }

    commands
      .entity(entity)
      .despawn_related::<RelatedTooltip<T>>()
      .with_related::<TooltipOf<T>>(
        tooltip.update(entity, &query)
      );
  }
}

fn add_hover_observer<T: Tooltip>(
  mut world: DeferredWorld,
  context: HookContext,
) {
  world
    .commands()
    .entity(context.entity)
    .observe(over_tooltip::<T>(context.entity))
    .observe(out_tooltip::<T>(context.entity));
}

fn over_tooltip<T: Tooltip>(
  entity: Entity,
) -> impl FnMut(Trigger<Pointer<Move>>, Query<&RelatedTooltip<T>>, Query<(&mut Visibility, &mut Node, &ComputedNode)>, Query<&Window, With<PrimaryWindow>>) {
  move |
    hover: Trigger<Pointer<Move>>,
    related_tooltip: Query<&RelatedTooltip<T>>,
    mut query: Query<(&mut Visibility, &mut Node, &ComputedNode)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
  | {
    if hover.target != entity { return; }
    let window = q_window.single().unwrap();
    let &RelatedTooltip { tooltip, ..} = related_tooltip.get(hover.target).unwrap();
    let Some(cursor_position) = window.cursor_position() else { return; };
    let (mut visibility, mut node, computed_node) = query.get_mut(tooltip).unwrap();
    *visibility = Visibility::Visible;
    if cursor_position.x < WIDTH / 2.0 {
      node.left = Val::Px(cursor_position.x);
    } else {
      node.left = Val::Px(cursor_position.x - computed_node.size.x * computed_node.inverse_scale_factor());
    }
    if cursor_position.y < HEIGHT / 2.0 {
      node.top = Val::Px(cursor_position.y);
    } else {
      node.top = Val::Px(cursor_position.y - computed_node.size.y * computed_node.inverse_scale_factor());
    }
  }
}

fn out_tooltip<T: Tooltip>(
  entity: Entity,
) -> impl FnMut(Trigger<Pointer<Out>>, Query<&RelatedTooltip<T>>, Query<&mut Visibility>) {
  move |
    hover: Trigger<Pointer<Out>>,
    related_tooltip: Query<&RelatedTooltip<T>>,
    mut nodes: Query<&mut Visibility>,
  | {
    if hover.target != entity { return; }
    let &RelatedTooltip { tooltip, ..} = related_tooltip.get(hover.target).unwrap();
    let mut visibility = nodes.get_mut(tooltip).unwrap();
    *visibility = Visibility::Hidden;
  }
}
