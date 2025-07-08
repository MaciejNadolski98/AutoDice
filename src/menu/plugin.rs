use bevy::{prelude::*, app::AppExit, ui::Interaction};
use crate::{dice::DiceTemplateBuilder, manage::plugin::{Coins, MyTeam, ShopRound}, states::GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Menu), spawn_menu)
      .add_systems(OnExit(GameState::Menu), despawn_menu)
      .add_systems(Update, button_actions.run_if(in_state(GameState::Menu)));
  }
}

#[derive(Component)]
enum ButtonAction {
    Play,
    Quit,
}

#[derive(Component)]
struct MenuScreen;

fn spawn_menu(
  mut commands: Commands,
) {
  commands.spawn((
    Name::new("Main Menu"),
    Node {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      align_items: AlignItems::Center,
      flex_direction: FlexDirection::Column,
      justify_content: JustifyContent::SpaceAround,
      ..default() 
    },
    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
    MenuScreen,
  ))
    .with_children(|commands| {
      commands.spawn((
        Name::new("Main menu title"),
        Text("AutoChess game thingy".to_string()),
        TextFont {
            font_size: 80.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 0.0, 0.0)),
    ));

      commands.spawn((
        Name::new("Play button"),
        Button,
        Node { align_items: AlignItems::Center, justify_content: JustifyContent::Center, width: Val::Percent(10.0), height: Val::Percent(10.0), ..default() },
        BackgroundColor(Color::srgb(0.5, 0.0, 0.0)),
        ButtonAction::Play,
      )).with_children(|commands| {
        commands.spawn((
          Text("Play".to_string()),
          TextFont { font_size: 50.0, ..default() },
          TextColor(Color::srgb(0.0, 0.0, 0.0)),
        ));
      });

      commands.spawn((
        Name::new("Quit button"),
        Button,
        Node { align_items: AlignItems::Center, justify_content: JustifyContent::Center, width: Val::Percent(10.0), height: Val::Percent(10.0), ..default() },
        BackgroundColor(Color::srgb(0.5, 0.0, 0.0)),
        ButtonAction::Quit,
      )).with_children(|commands| {
        commands.spawn((
          Text("Quit".to_string()),
          TextFont { font_size: 50.0, ..default() },
          TextColor(Color::srgb(0.0, 0.0, 0.0)),
        ));
      });
    });
}

fn despawn_menu(
  mut commands: Commands,
  menu: Query<Entity, With<MenuScreen>>,
) {
  commands.entity(menu.single().unwrap()).despawn();
}

fn button_actions(
  interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
  mut app_exit_events: EventWriter<AppExit>,
  mut commands: Commands,
) {
  for (interaction, button_action) in &interaction_query {
    if *interaction != Interaction::Pressed {
      continue;
    }

    match button_action {
      ButtonAction::Play => {
        commands.run_system_cached(new_game);
      }
      ButtonAction::Quit => { app_exit_events.write(AppExit::Success); }
    }
  }
}

fn new_game(
  mut shop_round: ResMut<ShopRound>,
  mut coins: ResMut<Coins>,
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  mut game_state: ResMut<NextState<GameState>>,
) {
  **coins = 5;
  **shop_round = 1;
  commands.spawn((
    Name::new("My team"),
    MyTeam,
  )).with_children(|commands| {
    for builder in [
      DiceTemplateBuilder::berserker(1),
      DiceTemplateBuilder::paladin(1),
      DiceTemplateBuilder::mage(1),
      DiceTemplateBuilder::cleric(1),
      DiceTemplateBuilder::rogue(1),
    ] {
      builder.spawn(commands, &mut images);
    }
  });
  game_state.set(GameState::Manage);
}
