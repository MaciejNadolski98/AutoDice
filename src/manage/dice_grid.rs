use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::{constants::GRID_FACE_SIZE, dice::{DiceTemplate, Face, FaceSource, Gridable}, manage::tile::Tile};

pub struct DiceGridPlugin;

impl Plugin for DiceGridPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (update_grids::<Tile>, update_grids::<DiceTemplate>));
  }
}

#[derive(Component, Reflect)]
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

#[derive(Component, Reflect, Clone)]
#[relationship_target(relationship = DiceGridOf, linked_spawn)]
pub struct DiceGrid {
  grid: Entity,
}

impl DiceGrid {
  pub fn grid(&self) -> Entity {
    self.grid
  }

  pub fn spawn<'a>(commands: &'a mut RelatedSpawnerCommands<ChildOf>, gridable: Entity) -> EntityCommands<'a> {
    commands
      .spawn((
        Name::new("Dice grid"),
        DiceGridOf::new(gridable),
        Node {
          display: Display::Grid,
          ..default()
        },
        Pickable::default(),
      ))
  }
}

fn update_grids<Faces: Gridable>(
  collections: Query<Entity, (With<Faces>, Changed<Children>)>,
  mut commands: Commands,
) {
  for collection in collections {
    commands.run_system_cached_with(update_grid::<Faces>, collection);
  }
}

pub fn update_grid<Faces: Gridable>(
  input: In<Entity>,
  world: &World,
  mut commands: Commands,
  collections: Query<(&Faces, &DiceGrid)>,
  faces: Query<&Face>,
  children: Query<&Children>,
) {
  let collection_entity = *input;
  info!("{:#?}",
    world
      .inspect_entity(collection_entity)
      .unwrap()
      .map(|info| info.name())
      .collect::<Vec<_>>()
  );
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
        let Face { image, .. } = faces.get(face).unwrap();
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
          Pickable::IGNORE,
        ));
      }
    });
}
