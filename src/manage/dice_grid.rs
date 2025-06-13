use bevy::prelude::*;

use crate::{constants::GRID_FACE_SIZE, dice::{DiceTemplate, Face, GridableFaceCollection}, manage::tile::Tile};

pub struct DiceGridPlugin;

impl Plugin for DiceGridPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (update_grids::<Tile>, update_grids::<DiceTemplate>));
  }
}

#[derive(Component)]
#[relationship(relationship_target = DiceGrid)]
pub struct DiceGridOf {
  collection: Entity,
}

impl DiceGridOf {
  pub fn new(collection: Entity) -> Self {
    Self {
      collection
    }
  }
}

#[derive(Component, Clone)]
#[relationship_target(relationship = DiceGridOf)]
pub struct DiceGrid {
  grid: Entity,
}

fn update_grids<Faces: GridableFaceCollection>(
  collections: Query<Entity, Changed<Faces>>,
  mut commands: Commands,
) {
  for collection in collections {
    commands.run_system_cached_with(update_grid::<Faces>, collection);
  }
}

pub fn update_grid<Faces: GridableFaceCollection>(
  input: In<Entity>,
  mut commands: Commands,
  templates: Query<(&Faces, &DiceGrid)>,
) {
  let (collection, DiceGrid { grid }) = templates.get(*input).unwrap();
  let faces = collection
    .gridded_faces().into_iter()
    .map(|(x, y, face)| (
      GridPlacement::start_span(x as i16, 1),
      GridPlacement::start_span(y as i16, 1),
      face,
    ));
  commands
    .entity(*grid)
    .despawn_related::<Children>()
    .with_children(|commands | {
      for (x, y, Face { image, .. }) in faces {
        commands.spawn((
          Name::new("Face"),
          Node {
            grid_column: x,
            grid_row: y,
            width: Val::Px(GRID_FACE_SIZE),
            height: Val::Px(GRID_FACE_SIZE),
            ..default()
          },
          ImageNode::from(image),
        ));
      }
    });
}
