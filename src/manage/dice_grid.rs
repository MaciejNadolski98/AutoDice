use bevy::prelude::*;

use crate::{dice::{DiceTemplate, Face}};

pub struct DiceGridPlugin;

impl Plugin for DiceGridPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, update_grids);
  }
}

#[derive(Component)]
#[relationship(relationship_target = DiceGrid)]
pub struct DiceGridOf {
  template: Entity,
}

impl DiceGridOf {
  pub fn new(template: Entity) -> Self {
    Self {
      template
    }
  }
}


#[derive(Component, Clone)]
#[relationship_target(relationship = DiceGridOf)]
pub struct DiceGrid {
  grid: Entity,
}

fn update_grids(
  templates: Query<Entity, Changed<DiceTemplate>>,
  mut commands: Commands,
) {
  for template in templates {
    commands.run_system_cached_with(update_grid, template);
  }
}

pub fn update_grid(
  input: In<Entity>,
  mut commands: Commands,
  templates: Query<(&DiceTemplate, &DiceGrid)>,
) {
  let (DiceTemplate { faces, .. }, DiceGrid { grid }) = templates.get(*input).unwrap();
  let grid_placements = [
    (2, 1),
    (1, 2),
    (2, 2),
    (3, 2),
    (2, 3),
    (2, 4),
  ].map(|(x, y)| (
    GridPlacement::start_span(x, 1), 
    GridPlacement::start_span(y, 1),
  ));
  commands
    .entity(*grid)
    .despawn_related::<Children>()
    .with_children(|commands | {
      for ((x, y), Face { image, .. }) in grid_placements.iter().zip(faces.iter()) {
        commands.spawn((
          Name::new("Face"),
          Node {
            grid_column: *x,
            grid_row: *y,
            ..default()
          },
          ImageNode::from(image.clone()),
        ));
      }
    });
}
