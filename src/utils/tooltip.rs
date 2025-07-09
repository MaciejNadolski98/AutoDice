use bevy::{ecs::{component::HookContext, world::DeferredWorld}, prelude::*, render::view::RenderLayers, window::PrimaryWindow};

use crate::constants::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH, TOOLTIP_LAYER};

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<TooltipsEnabled>();
  }
}

#[derive(Resource, Default)]
pub struct TooltipsEnabled(bool);

pub fn toggle_tooltips(
  mut tooltips: ResMut<TooltipsEnabled>,
  query: Query<&mut Visibility, With<TooltipOf>>,
) {
  if tooltips.0 {
    for mut visibility in query {
      *visibility = Visibility::Hidden;
    }
  }
  tooltips.0 = !tooltips.0;
}

#[derive(Component)]
#[relationship(relationship_target = Tooltip)]
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
  RenderLayers = TOOLTIP_LAYER,
)]
pub struct TooltipOf {
  #[relationship]
  entity: Entity,
}

#[derive(Component)]
#[relationship_target(relationship = TooltipOf, linked_spawn)]
#[component(on_add = add_hover_observer)]
#[require(Pickable::default())]
pub struct Tooltip {
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
) -> impl IntoSystem<Trigger<'static, Pointer<Move>>, (), ()> {
  let closure = move |
    hover: Trigger<Pointer<Move>>,
    related_tooltip: Query<&Tooltip>,
    mut query: Query<(&mut Visibility, &mut Node, &ComputedNode)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    enabled: Res<TooltipsEnabled>,
  | {
    if hover.target != entity || !enabled.0 { return; }
    let window = q_window.single().unwrap();
    let &Tooltip { tooltip, ..} = related_tooltip.get(hover.target).unwrap();
    let Some(cursor_position) = window.cursor_position() else { return; };
    let (mut visibility, mut node, computed_node) = query.get_mut(tooltip).unwrap();
    *visibility = Visibility::Visible;
    if cursor_position.x < RESOLUTION_WIDTH / 2.0 {
      node.left = Val::Px(cursor_position.x);
    } else {
      node.left = Val::Px(cursor_position.x - computed_node.size.x * computed_node.inverse_scale_factor());
    }
    if cursor_position.y < RESOLUTION_HEIGHT / 2.0 {
      node.top = Val::Px(cursor_position.y);
    } else {
      node.top = Val::Px(cursor_position.y - computed_node.size.y * computed_node.inverse_scale_factor());
    }
  };

  IntoSystem::into_system(closure)
}

fn out_tooltip(
  entity: Entity,
) -> impl IntoSystem<Trigger<'static, Pointer<Out>>, (), ()> {
  let closure = move |
    hover: Trigger<Pointer<Out>>,
    related_tooltip: Query<&Tooltip>,
    mut nodes: Query<&mut Visibility>,
  | {
    if hover.target != entity { return; }
    let &Tooltip { tooltip, ..} = related_tooltip.get(hover.target).unwrap();
    let mut visibility = nodes.get_mut(tooltip).unwrap();
    *visibility = Visibility::Hidden;
  };

  IntoSystem::into_system(closure)
}
