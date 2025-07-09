use bevy::prelude::*;
use crate::{battle::{clean_up_game, Challenge}, constants::{dice_texture::TARGET_SIZE, ui::{BUTTON_SIZE, COINS_NUMBER_SIZE, REFRESH_BUTTON_SIZE, ROUND_NUMBER_SIZE}, DICE_SIZE, REFRESH_PRICE, SHOP_ITEMS_COUNT}, dice::{spawn_synergy_displays, DiceTemplate, Face, FaceSource, HealthBar}, loading_screen::AssetStore, manage::{dice_grid::{DiceGrid, DiceGridOf, DiceGridPlugin}, tile::{Buyable, Tile}}, states::GameState};

pub struct ManagePlugin;

impl Plugin for ManagePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<Coins>()
      .add_plugins(DiceGridPlugin)
      .init_resource::<ShopRound>()
      .add_systems(OnEnter(GameState::Manage), (spawn_enemy, spawn_shop, spawn_manage).chain())
      .add_systems(OnExit(GameState::Manage), (despawn_shop, despawn_manage).chain())
      .add_systems(Update, button_actions.run_if(in_state(GameState::Manage)))
      .add_systems(Update, (update_coins, refresh_shop, update_shop_spots).run_if(in_state(GameState::Manage)));
  }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Coins(u32);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct ShopRound(u32);

#[derive(Component)]
struct ManageScreen;

#[derive(Component)]
enum ButtonAction {
    Battle,
    BackToMenu,
}

#[derive(Component)]
struct CoinsDisplay;

fn update_coins(
  display: Single<&mut Text, With<CoinsDisplay>>,
  coins: Res<Coins>,
) {
  if !coins.is_changed() { return; }

  *display.into_inner() = Text::new(format!("Coins: {}", **coins));
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
  )).with_children(|commands| {
    for builder in Challenge::new(shop_round.0).0 {
      builder.spawn(commands, &mut images);
    }
  });
}

#[derive(Component)]
pub struct Shop;

#[derive(Component)]
#[relationship(relationship_target = ShopSpotOf)]
pub struct ShopSpot(Entity);

#[derive(Component)]
#[relationship_target(relationship = ShopSpot)]
pub struct ShopSpotOf(Entity);

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
        commands
          .spawn((
            Name::new("Shop spot"),
          ))
          .with_children(|commands| {
            Tile::spawn(&mut images, commands);
          });
      }
    });
}

fn update_shop_spots(
  mut commands: Commands,
  shop_spots: Query<(&Children, &ShopSpotOf), Changed<Children>>,
) {
  for (children, ShopSpotOf(spot)) in shop_spots {
    if children.is_empty() { continue };
    assert!(children.len() == 1);
    let child = children[0];

    commands
      .entity(*spot)
      .with_children(|commands| {
        let grid_tile = DiceGrid::spawn(commands, child).id();

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
  }
}

#[derive(Component)]
struct RefreshButton;

#[allow(clippy::type_complexity)]
fn refresh_shop(
  mut commands: Commands,
  shop_spots: Query<Entity, With<ShopSpotOf>>,
  mut images: ResMut<Assets<Image>>,
  button: Option<Single<&Interaction, (With<RefreshButton>, Changed<Interaction>)>>,
  mut coins: ResMut<Coins>,
) {
  if button.is_none() || **button.unwrap() != Interaction::Pressed || **coins < REFRESH_PRICE {
    return;
  }
  **coins -= REFRESH_PRICE;
  for spot in shop_spots {
    commands
      .entity(spot)
      .despawn_related::<Children>()
      .with_children(|commands| {
        Tile::spawn(&mut images, commands);
      });
  }
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
  shop_round: Res<ShopRound>,
  asset_store: Res<AssetStore>,
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
        Name::new("Coins display"),
        Node {
          position_type: PositionType::Absolute,
          bottom: Val::Px(0.0),
          right: Val::Px(0.0),
          ..default()
        },
        Text::new(""),
        TextFont {
          font_size: COINS_NUMBER_SIZE,
          ..default()
        },
        TextColor(Color::BLACK),
        ZIndex(1),
        CoinsDisplay,
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
      )).with_children(|commands| {
        for &template in *my_team {
          commands.spawn((
            Name::new("Dice template spot"),
            Node::default(),
          )).with_children(|commands| {
            DiceGrid::spawn(commands, template);
            commands.spawn((
              Name::new("Health bar container"),
              Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(-0.2 * DICE_SIZE),
                margin: UiRect { left: Val::Auto, right: Val::Auto, ..default() },
                ..default()
              },
            )).with_children(|commands| {
              HealthBar::spawn(commands, template);
            });
          });
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
        commands.spawn((
          Name::new("Refresh button"),
          Button,
          RefreshButton,
          Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: REFRESH_BUTTON_SIZE,
            height: REFRESH_BUTTON_SIZE,
            ..default()
          },
          ImageNode::new(asset_store.get("ui/refresh.png")),
        ));


        for &shop_spot in *shop {
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
              ShopSpot(shop_spot),
              Pickable::IGNORE,
            ));
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

fn drag_tile(tile: Entity) -> impl IntoSystem<Trigger<'static, Pointer<Drag>>, (), ()> { 
  let closure = move |
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
  };

  IntoSystem::into_system(closure)
}

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

#[allow(clippy::too_many_arguments)]
fn overlap_tile_template(
  grid: In<Entity>,
  mut commands: Commands,
  transforms: Query<&GlobalTransform>,
  children: Query<&Children>,
  grids: Query<&DiceGrid, With<DiceTemplate>>,
  my_team: Single<&Children, With<MyTeam>>,
  overlap_indicators: Query<Entity, With<OverlapIndicator>>,
  computed: Query<&ComputedNode>,
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
        let tile_size = computed.get(template_face_node).unwrap().size.x;
        if face_position.distance(template_face_position) < tile_size / 2.0 {
          matches.push((face_node, (template, template_face_node)));
        }
      }
    }
  }
  if matches.is_empty() {
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

#[allow(clippy::too_many_arguments)]
fn apply_tile(
  input: In<OverlapTileTemplateOutput>,
  mut nodes: Query<&mut Node>,
  face_sources: Query<&FaceSource>,
  mut faces: Query<&mut Face>,
  grids: Query<&DiceGridOf>,
  mut coins: ResMut<Coins>,
  tiles: Query<&Tile>,
  mut commands: Commands,
) {
  let In(OverlapTileTemplateOutput { grid, matched, matches }) = input;
  let tile = grids.get(grid).unwrap().collection();
  let price = tiles.get(tile).unwrap().price();
  if !matched || **coins < price {
    let mut node = nodes.get_mut(grid).unwrap();
    node.position_type = PositionType::Relative;
    node.left = Val::Auto;
    node.top = Val::Auto;
    return;
  }
  **coins -= price;

  for (tile_node, template_node) in matches {
    let tile_face = face_sources.get(tile_node).unwrap().source();
    let template_face = face_sources.get(template_node).unwrap().source();

    let tile_face = faces.get(tile_face).unwrap().clone();
    let mut template_face = faces.get_mut(template_face).unwrap();
    template_face.prototype.action = tile_face.prototype.action;
    template_face.prototype.pips = tile_face.prototype.pips;
  }
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

#[allow(clippy::type_complexity)]
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
