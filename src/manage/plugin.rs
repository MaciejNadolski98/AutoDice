use bevy::prelude::*;
use crate::{battle::{clean_up_game, Challenge}, constants::{dice_texture::TARGET_SIZE, ui::{BUTTON_SIZE, ROUND_NUMBER_SIZE}, GRID_FACE_SIZE, SHOP_ITEMS_COUNT}, dice::{spawn_synergy_displays, DiceTemplate, Face, FaceSource}, manage::{dice_grid::{update_grid, DiceGrid, DiceGridOf, DiceGridPlugin}, tile::Tile}, states::GameState};

pub struct ManagePlugin;

impl Plugin for ManagePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(DiceGridPlugin)
      .init_resource::<ShopRound>()
      .add_systems(OnEnter(GameState::Manage), (spawn_enemy, spawn_shop, spawn_manage).chain())
      .add_systems(OnExit(GameState::Manage), (despawn_shop, despawn_manage).chain())
      .add_systems(Update, button_actions.run_if(in_state(GameState::Manage)));
  }
}

#[derive(Resource, Default)]
pub struct ShopRound(pub u32);

#[derive(Component)]
struct ManageScreen;

#[derive(Component)]
enum ButtonAction {
    Battle,
    BackToMenu,
}

#[derive(Component)]
pub struct MyTeam;

#[derive(Component)]
pub struct EnemyTeam;

pub fn spawn_enemy(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  enemy_team: Option<Single<Entity, With<EnemyTeam>>>,
  shop_round: Res<ShopRound>,
) {
  if let Some(entity) = enemy_team {
    commands.entity(*entity).despawn();
  }
  commands.spawn((
    Name::new("Enemy team"),
    EnemyTeam,
  )).with_children(|mut commands| {
    for builder in Challenge::new(shop_round.0).0 {
      builder.spawn(&mut commands, &mut images);
    }
  });
}

#[derive(Component)]
pub struct Shop;

fn spawn_shop(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
) {
  commands.spawn((
      Name::new("Shop"),
      Shop,
    ))
    .with_children(|commands| {
      for _ in 0..SHOP_ITEMS_COUNT {
        commands.spawn(
          Name::new("Shop spot"),
        ).with_children(|mut commands| {
          Tile::spawn(&mut images, &mut commands);
        });
      }
    });
}

fn despawn_shop(
  mut commands: Commands,
  shop: Single<Entity, With<Shop>>,
) {
  commands
    .entity(*shop)
    .despawn();
}

fn spawn_manage(
  mut commands: Commands,
  my_team: Single<&Children, With<MyTeam>>,
  shop: Single<&Children, With<Shop>>,
  children: Query<&Children>,
  shop_round: Res<ShopRound>,
) {
  commands.spawn((
    Name::new("Manage"),
    Node {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      flex_direction: FlexDirection::Column,
      justify_content: JustifyContent::Start,
      ..default() 
    },
    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
    ManageScreen,
  )).with_children(|commands| {
    commands.spawn((
      Name::new("Shop area"),
      Node {
        width: Val::Percent(100.0),
        height: Val::Percent(80.0),
        flex_direction: FlexDirection::Row,
        ..default()
      },
    )).with_children(|commands|{
      commands.spawn((
        Name::new("Round number"),
        Node {
          position_type: PositionType::Absolute,
          top: Val::Px(0.0),
          margin: UiRect {
            left: Val::Auto,
            right: Val::Auto,
            ..default()
          },
          ..default()
        },
        Text::new(format!("Round {}/4", shop_round.0)),
        TextFont {
          font_size: ROUND_NUMBER_SIZE,
          ..default()
        },
        TextColor(Color::BLACK),
        ZIndex(1),
      ));

      commands.spawn((
        Name::new("Synergies"),
        Node {
          position_type: PositionType::Absolute,
          width: Val::Percent(10.0),
          height: Val::Percent(10.0),
          flex_direction: FlexDirection::Column,
          justify_content: JustifyContent::Start,
          align_items: AlignItems::Center,
          ..default()
        },
        ZIndex(1),
      )).with_children(spawn_synergy_displays);

      commands.spawn((
        Name::new("Dice display"),
        Node {
          width: Val::Percent(70.0),
          justify_content: JustifyContent::Center,
          flex_wrap: FlexWrap::Wrap,
          align_content: AlignContent::SpaceAround,
          align_items: AlignItems::Center,
          row_gap: Val::Px(-TARGET_SIZE),
          column_gap: Val::Px(TARGET_SIZE / 8.0),
          ..default()
        },
        BackgroundColor(Color::srgb(0.6, 0.4, 0.2)),
      )).with_children(|mut commands| {
        for &template in *my_team {
          DiceGrid::spawn(&mut commands, template);

          commands.commands().run_system_cached_with(update_grid::<DiceTemplate>, template);
        }
      });
      commands.spawn((
        Name::new("Shop"),
        Node {
          width: Val::Percent(30.0),
          flex_direction: FlexDirection::Column,
          ..default()
        },
        BackgroundColor(Color::srgb(0.8, 0.6, 0.4)),
      )).with_children(|commands| {
        for &shop_spot in *shop {
          let &tile = children.get(shop_spot).unwrap().first().unwrap();
          commands
            .spawn((
              Name::new("Shop spot"),
              Node {
                width: Val::Percent(100.0),
                height: Val::Percent(25.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
              },
              Pickable::IGNORE,
            ))
            .with_children(|mut commands| {
              let grid_tile = DiceGrid::spawn(&mut commands, tile).id();

              commands.commands()
                .entity(grid_tile)
                .observe(drag_tile(grid_tile))
                .observe(
                  on_drag(grid_tile)
                    .pipe(overlap_tile_template)
                    .pipe(mark_faces)
                )
                .observe(
                  on_release(grid_tile)
                    .pipe(overlap_tile_template)
                    .pipe(apply_tile)
                );
            });

          commands.commands().run_system_cached_with(update_grid::<Tile>, tile);
        }
      });
    });

    commands.spawn((
      Name::new("Bottom menu"),
      Node {
        width: Val::Percent(100.0),
        height: Val::Percent(20.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        ..default() 
      },
      BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
    )).with_children(|commands| {
      commands.spawn((
        Name::new("Left side"),
        Node {
          align_items: AlignItems::Center,
          height: Val::Percent(100.0),
          flex_grow: 1.0,
          left: Val::Percent(10.0),
          ..default()
        },
      )).with_children(|commands| {
        commands.spawn((
          Name::new("Back to menu button"),
          Button,
          Node {
            width: BUTTON_SIZE,
            height: Val::Percent(60.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
          },
          BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
          ButtonAction::BackToMenu,
        )).with_children(|commands| {
          commands.spawn((
            Text("Go Back".to_string()),
            TextFont { font_size: 30.0, ..default() },
            TextColor(Color::srgb(0.0, 0.0, 0.0)),
          ));
        });
      });

      commands.spawn((
        Name::new("Right side"),
        Node {
          height: Val::Percent(100.0),
          flex_grow: 1.0,
          flex_direction: FlexDirection::RowReverse,
          align_items: AlignItems::Center,
          right: Val::Percent(10.0),
          ..default()
        },
      )).with_children(|commands| {
        commands.spawn((
          Name::new("Battle button"),
          Button,
          Node {
            width: BUTTON_SIZE,
            height: Val::Percent(60.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
          },
          BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
          ButtonAction::Battle,
        )).with_children(|commands| {
          commands.spawn((
            Text("Battle!".to_string()),
            TextFont { font_size: 30.0, ..default() },
            TextColor(Color::srgb(0.0, 0.0, 0.0)),
          ));
        });
      });
    });
  });
}

fn drag_tile(tile: Entity) -> impl Fn(Trigger<Pointer<Drag>>, Query<(&mut Node, &ChildOf)>, Query<&ComputedNode>) { move |
  drag: Trigger<Pointer<Drag>>,
  mut tiles: Query<(&mut Node, &ChildOf)>,
  computed_nodes: Query<&ComputedNode>,
| {
  fn size(node: &ComputedNode) -> Vec2 {
    node.size * node.inverse_scale_factor
  }
  let delta = drag.delta;

  let (mut node, &ChildOf(parent)) = tiles.get_mut(tile).unwrap();
  let parent_size = size(computed_nodes.get(parent).unwrap());
  let size = size(computed_nodes.get(tile).unwrap());
  node.position_type = PositionType::Absolute;
  match (node.left, node.top) {
    (Val::Px(x), Val::Px(y)) => {
      node.left = Val::Px(x + delta.x);
      node.top = Val::Px(y + delta.y);
    },
    (_, _) => {
      let Vec2 { x, y } = parent_size / 2.0 - size / 2.0;
      node.left = Val::Px(x + delta.x);
      node.top = Val::Px(y + delta.y);
    },
  }
}}

fn on_drag(tile: Entity) -> impl Fn(Trigger<Pointer<Drag>>) -> Entity {move |
  _trigger: Trigger<Pointer<Drag>>,
| {
  tile
}}

fn on_release(tile: Entity) -> impl Fn(Trigger<Pointer<Released>>) -> Entity {move |
  _trigger: Trigger<Pointer<Released>>,
| {
  tile
}}

struct OverlapTileTemplateOutput {
  grid: Entity,
  matched: bool,
  matches: Vec<(Entity, Entity)>,
}

fn overlap_tile_template(
  grid: In<Entity>,
  mut commands: Commands,
  transforms: Query<&GlobalTransform>,
  children: Query<&Children>,
  grids: Query<&DiceGrid, With<DiceTemplate>>,
  my_team: Single<&Children, With<MyTeam>>,
  overlap_indicators: Query<Entity, With<OverlapIndicator>>,
) -> OverlapTileTemplateOutput {
  let grid = *grid;
  // Cleanup
  for entity in overlap_indicators {
    commands.entity(entity).despawn();
  }

  // List all matches between tile faces and template faces
  let mut matches = Vec::new();
  for &face_node in children.get(grid).unwrap() {
    let face_position = transforms.get(face_node).unwrap().translation();
    for &template in *my_team {
      let template_grid = grids.get(template).unwrap().grid();
      for &template_face_node in children.get(template_grid).unwrap() {
        let template_face_position = transforms.get(template_face_node).unwrap().translation();
        if face_position.distance(template_face_position) < GRID_FACE_SIZE {
          matches.push((face_node, (template, template_face_node)));
        }
      }
    }
  }
  if matches.len() == 0 {
    return OverlapTileTemplateOutput { grid, matched: false, matches: Vec::new() };
  }

  let mut valid = true;
  if matches.len() != children.get(grid).unwrap().len() {
    valid = false;
  }

  let templates: Vec<Entity> = matches
    .iter()
    .map(|(_, (template, _))| *template)
    .collect();
  let first = templates[0];
  for x in &templates {
    if *x != first {
      valid = false;
      break;
    }
  }
  
  OverlapTileTemplateOutput {
    grid,
    matched: valid,
    matches: matches.iter().map(|(face, (_, template_face_node))| (*face, *template_face_node)).collect(),
  }
}

const GREEN: Color = Color::linear_rgb(0.0, 1.0, 0.0);
const RED: Color = Color::linear_rgb(1.0, 0.0, 0.0);

fn mark_faces(
  input: In<OverlapTileTemplateOutput>,
  mut commands: Commands,
) {
  let In(OverlapTileTemplateOutput { matched, matches , ..}) = input;
  let color = if matched { GREEN } else { RED };
  for (_, template) in matches {
    commands
      .entity(template)
      .with_child((
        Node {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          ..default()
        },
        BackgroundColor::from(color),
        ZIndex(1),
        OverlapIndicator,
      ));
  }
}

fn apply_tile(
  input: In<OverlapTileTemplateOutput>,
  mut nodes: Query<&mut Node>,
  face_sources: Query<&FaceSource>,
  mut faces: Query<&mut Face>,
  grids: Query<&DiceGridOf>,
  mut commands: Commands,
) {
  let In(OverlapTileTemplateOutput { grid, matched, matches }) = input;
  if !matched {
    let mut node = nodes.get_mut(grid).unwrap();
    node.position_type = PositionType::Relative;
    node.left = Val::Auto;
    node.top = Val::Auto;
    return;
  }

  for (tile_node, template_node) in matches {
    let tile_face = face_sources.get(tile_node).unwrap().source();
    let template_face = face_sources.get(template_node).unwrap().source();

    let tile_face = faces.get(tile_face).unwrap().clone();
    let mut template_face = faces.get_mut(template_face).unwrap();
    template_face.prototype.action = tile_face.prototype.action;
    template_face.prototype.pips = tile_face.prototype.pips;
  }
  let tile = grids.get(grid).unwrap().collection();
  commands.entity(tile).despawn();
}

#[derive(Component)]
struct OverlapIndicator;

fn despawn_manage(
  screen: Query<Entity, With<ManageScreen>>,
  mut commands: Commands,
) {
  commands.entity(screen.single().unwrap()).despawn();
}

fn button_actions(
  interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
  mut game_state: ResMut<NextState<GameState>>,
  mut commands: Commands,
) {
  for (interaction, button_action) in &interaction_query {
    if *interaction != Interaction::Pressed {
      continue;
    }

    match button_action {
      ButtonAction::BackToMenu => {
        commands.run_system_cached(clean_up_game);
        game_state.set(GameState::Menu);
      }
      ButtonAction::Battle => {
        game_state.set(GameState::Battle);
      }
    }
  }
}
