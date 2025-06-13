use bevy::prelude::*;
use crate::{constants::{dice_texture::TARGET_SIZE, ui::BUTTON_SIZE, DICE_COUNT, SHOP_ITEMS_COUNT}, dice::DiceTemplate, manage::{dice_grid::{update_grid, DiceGridOf}, tile::Tile}, states::GameState};

pub struct ManagePlugin;

impl Plugin for ManagePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Manage), (spawn_shop, spawn_manage).chain())
      .add_systems(OnExit(GameState::Manage), despawn_manage)
      .add_systems(Update, button_actions.run_if(in_state(GameState::Manage)));
  }
}

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

pub fn spawn_teams(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>
) {
  commands.spawn((
    Name::new("My team"),
    MyTeam,
  )).with_children(|commands| {
    for _ in 0..DICE_COUNT {
      commands.spawn((
        DiceTemplate::generate(&mut images),
      ));
    }
  });

  commands.spawn((
    Name::new("Enemy team"),
    EnemyTeam,
  )).with_children(|commands| {
    for _ in 0..DICE_COUNT {
      commands.spawn((
        DiceTemplate::generate(&mut images),
      ));
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
      Shop,
    ))
    .with_children(|commands| {
      for _ in 0..SHOP_ITEMS_COUNT {
        commands.spawn((
          Tile::generate(&mut images),
        ));
      }
    });
}

fn spawn_manage(
  mut commands: Commands,
  my_team: Single<&Children, With<MyTeam>>,
  shop: Single<&Children, With<Shop>>,
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
            Name::new("Dice grid"),
            DiceGridOf::new(template),
            Node {
              display: Display::Grid,
              ..default()
            },
          ));

          commands.commands().run_system_cached_with(update_grid::<DiceTemplate>, template);
        }
      });
      commands.spawn((
        Name::new("Shop"),
        Node {
          width: Val::Percent(30.0),
          justify_content: JustifyContent::SpaceAround,
          flex_direction: FlexDirection::Column,
          align_items: AlignItems::Center,
          ..default()
        },
        BackgroundColor(Color::srgb(0.8, 0.6, 0.4)),
      )).with_children(|commands| {
        for &tile in *shop {
          commands.spawn((
            Name::new("Dice grid"),
            DiceGridOf::new(tile),
            Node {
              display: Display::Grid,
              ..default()
            },
          ));

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

fn despawn_manage(
  screen: Query<Entity, With<ManageScreen>>,
  mut commands: Commands,
) {
  commands.entity(screen.single().unwrap()).despawn();
}

fn button_actions(
  interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
  mut game_state: ResMut<NextState<GameState>>,
) {
  for (interaction, button_action) in &interaction_query {
    if *interaction != Interaction::Pressed {
      continue;
    }

    match button_action {
      ButtonAction::BackToMenu => {
        game_state.set(GameState::Menu);
      }
      ButtonAction::Battle => {
        game_state.set(GameState::Battle);
      }
    }
  }
}
