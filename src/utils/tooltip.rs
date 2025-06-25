use bevy::{ecs::{component::HookContext, world::DeferredWorld}, prelude::*, window::PrimaryWindow};

use crate::constants::{HEIGHT, WIDTH};

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
  fn build(&self, _app: &mut App) {
  }
}

#[derive(Component)]
#[relationship(relationship_target = RelatedTooltip)]
#[require(
  BackgroundColor(Color::WHITE),
  Outline {
    width: Val::Px(1.0),
    color: Color::BLACK,
    ..default()
  },
  TextColor(Color::BLACK),
  Visibility::Hidden,
  Pickable::IGNORE,
)]
pub struct TooltipOf {
  #[relationship]
  entity: Entity,
}

#[derive(Component)]
#[relationship_target(relationship = TooltipOf, linked_spawn)]
#[component(on_add = add_hover_observer)]
pub struct RelatedTooltip {
  #[relationship]
  tooltip: Entity,
}

fn add_hover_observer(
  mut world: DeferredWorld,
  context: HookContext,
) {
  world
    .commands()
    .entity(context.entity)
    .observe(over_tooltip(context.entity))
    .observe(out_tooltip(context.entity));
}

fn over_tooltip(
  entity: Entity,
) -> impl FnMut(Trigger<Pointer<Move>>, Query<&RelatedTooltip>, Query<(&mut Visibility, &mut Node, &ComputedNode)>, Query<&Window, With<PrimaryWindow>>) {
  move |
    hover: Trigger<Pointer<Move>>,
    related_tooltip: Query<&RelatedTooltip>,
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

fn out_tooltip(
  entity: Entity,
) -> impl FnMut(Trigger<Pointer<Out>>, Query<&RelatedTooltip>, Query<&mut Visibility>) {
  move |
    hover: Trigger<Pointer<Out>>,
    related_tooltip: Query<&RelatedTooltip>,
    mut nodes: Query<&mut Visibility>,
  | {
    if hover.target != entity { return; }
    let &RelatedTooltip { tooltip, ..} = related_tooltip.get(hover.target).unwrap();
    let mut visibility = nodes.get_mut(tooltip).unwrap();
    *visibility = Visibility::Hidden;
  }
}
