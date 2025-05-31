use bevy::prelude::*;
use crate::{constants::MAX_DICE_COUNT, dice::DiceTemplate, states::GameState};

pub struct ManagePlugin;

impl Plugin for ManagePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<DiceData>()
      .add_systems(OnEnter(GameState::Manage), spawn_manage)
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

fn spawn_manage(
  mut commands: Commands,
) {
  commands.spawn((
    Name::new("Manage"),
    Node { 
      width: Val::Percent(100.0), 
      height: Val::Percent(100.0),
      flex_direction: FlexDirection::Column,
      justify_content: JustifyContent::End,
      ..default() 
    },
    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
    ManageScreen,
  )).with_children(|commands| {
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
        Name::new("Back to menu button"),
        Button,
        Node { 
          width: Val::Percent(10.0), 
          height: Val::Percent(60.0), 
          left: Val::Px(50.0),
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

      commands.spawn((
        Name::new("Battle button"),
        Button,
        Node { 
          width: Val::Percent(10.0), 
          height: Val::Percent(60.0),
          right: Val::Px(50.0),
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
}

fn despawn_manage(
  screen: Query<Entity, With<ManageScreen>>,
  mut commands: Commands,
) {
  commands.entity(screen.single().unwrap()).despawn();
}

#[derive(Resource, Default)]
pub struct DiceData {
  pub team1: Vec<DiceTemplate>,
  pub team2: Vec<DiceTemplate>,
}

fn button_actions(
  interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
  mut game_state: ResMut<NextState<GameState>>,
  mut dice_data: ResMut<DiceData>,
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
        *dice_data = DiceData {
          team1: (0..MAX_DICE_COUNT).into_iter().map(|_| DiceTemplate::generate()).collect(),
          team2: (0..MAX_DICE_COUNT).into_iter().map(|_| DiceTemplate::generate()).collect(),
        };
        game_state.set(GameState::Battle);
      }
    }
  }
}
