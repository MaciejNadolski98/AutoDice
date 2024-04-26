use bevy::prelude::*;
use crate::states::GameState;

pub struct ManagePlugin;

impl Plugin for ManagePlugin {
  fn build(&self, app: &mut App) {
    app
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
  info!("In Manage state");
  commands.spawn((
    NodeBundle {
      style: Style { 
        width: Val::Percent(100.0), 
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::End,
        ..default() 
      },
      background_color: Color::GRAY.into(),
      ..default()
    },
    ManageScreen,
    Name::new("Manage"),
  )).with_children(|commands| {
    // Bottom menu
    commands.spawn((
      NodeBundle {
        style: Style { 
          width: Val::Percent(100.0), 
          height: Val::Percent(20.0), 
          flex_direction: FlexDirection::Row,
          justify_content: JustifyContent::SpaceBetween,
          align_items: AlignItems::Center,
          ..default() 
        },
        background_color: Color::DARK_GRAY.into(),
        ..default()
      },
    )).with_children(|commands| {
      commands.spawn((
        ButtonBundle {
          style: Style { 
            width: Val::Percent(10.0), 
            height: Val::Percent(60.0), 
            left: Val::Px(50.0),
            justify_content: JustifyContent::Center, 
            align_items: AlignItems::Center, 
            ..default() 
          },
          background_color: Color::RED.into(),
          ..default()
        },
        ButtonAction::BackToMenu,
      )).with_children(|commands| {
        commands.spawn((
          TextBundle::from_section("Go Back", TextStyle { font_size: 30.0, color: Color::BLACK.into(), ..default() } ),
        ));
      });

      commands.spawn((
        ButtonBundle {
          style: Style { 
            width: Val::Percent(10.0), 
            height: Val::Percent(60.0),
            right: Val::Px(50.0),
            justify_content: JustifyContent::Center, 
            align_items: AlignItems::Center, 
            ..default() 
          },
          background_color: Color::RED.into(),
          ..default()
        },
        ButtonAction::Battle,
      )).with_children(|commands| {
        commands.spawn((
          TextBundle::from_section("Battle!", TextStyle { font_size: 30.0, color: Color::BLACK.into(), ..default() } ),
        ));
      });
    });
  });
}

fn despawn_manage(
  screen: Query<Entity, With<ManageScreen>>,
  mut commands: Commands,
) {
  commands.entity(screen.single()).despawn_recursive();
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
        // initialize battle data
      }
    }
  }
}
