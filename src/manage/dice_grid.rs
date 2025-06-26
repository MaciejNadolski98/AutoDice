use bevy::{ecs::relationship::{RelatedSpawnerCommands, Relationship}, prelude::*};

use crate::{constants::GRID_FACE_SIZE, dice::{Dice, DiceTemplate, Face, FaceSource, Gridable}, manage::tile::Tile, utils::tooltip::Tooltip};

pub struct DiceGridPlugin;

impl Plugin for DiceGridPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (update_grids::<Tile>, update_grids::<DiceTemplate>, update_grids::<Dice>));
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

  pub fn collection(&self) -> Entity {
    self.collection
  }
}

#[derive(Component, Clone)]
#[relationship_target(relationship = DiceGridOf, linked_spawn)]
pub struct DiceGrid {
  grid: Entity,
}

impl DiceGrid {
  pub fn grid(&self) -> Entity {
    self.grid
  }

  pub fn spawn<'a, R: Relationship>(commands: &'a mut RelatedSpawnerCommands<R>, gridable: Entity) -> EntityCommands<'a> {
    commands
      .spawn((
        Name::new("Dice grid"),
        DiceGridOf::new(gridable),
        Node {
          display: Display::Grid,
          ..default()
        },
      ))
  }
}

fn update_grids<Faces: Gridable>(
  changed_face: Query<&ChildOf, Changed<Face>>,
  collection: Query<(), (With<Faces>, With<DiceGrid>)>,
  mut commands: Commands,
) {
  for ChildOf(collection_entity) in changed_face {
    if collection.get(*collection_entity).is_ok() {
      commands.run_system_cached_with(update_grid::<Faces>, *collection_entity);
    }
  }
}

pub fn update_grid<Faces: Gridable>(
  input: In<Entity>,
  mut commands: Commands,
  collections: Query<(&Faces, &DiceGrid)>,
  faces: Query<&Face>,
  children: Query<&Children>,
) {
  let collection_entity = *input;
  let (collection, DiceGrid { grid: grid_entity }) = collections.get(collection_entity).unwrap();
  let grid = collection
    .grid()
    .into_iter()
    .zip(children.get(collection_entity).unwrap())
    .map(|((x, y), face_entity)| (
      GridPlacement::start_span(x as i16, 1),
      GridPlacement::start_span(y as i16, 1),
      *face_entity,
    ));
  commands
    .entity(*grid_entity)
    .despawn_related::<Children>()
    .with_children(|commands | {
      for (x, y, face) in grid {
        let Face { prototype, image } = faces.get(face).unwrap();
        commands.spawn((
          Name::new("Face"),
          Node {
            grid_column: x,
            grid_row: y,
            width: Val::Px(GRID_FACE_SIZE),
            height: Val::Px(GRID_FACE_SIZE),
            ..default()
          },
          FaceSource::new(face),
          ImageNode::from(image.clone()),
          related!(Tooltip[(
            Name::new("Face tooltip"),
            Text::new(prototype.description()),
          )]),
        ));
      }
    });
}
