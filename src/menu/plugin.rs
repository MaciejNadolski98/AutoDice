use bevy::{prelude::*, app::AppExit, ui::Interaction};
use crate::states::GameState;
use crate::camera::menu_camera::{spawn_menu_camera, despawn_menu_camera};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Menu), (spawn_menu, spawn_menu_camera))
      .add_systems(OnExit(GameState::Menu), (despawn_menu, despawn_menu_camera))
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
    NodeBundle {
      style: Style { 
        width: Val::Percent(100.0), 
        height: Val::Percent(100.0), 
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::SpaceAround,
        ..default() 
      },
      background_color: Color::GRAY.into(),
      ..default()
    },
    MenuScreen,
    Name::new("Main Menu"),
  ))
    .with_children(|commands| {
      // Main menu text
      commands.spawn(
        TextBundle::from_section(
          "AutoChess game thingy",
          TextStyle {
              font_size: 80.0,
              color: Color::BLACK.into(),
              ..default()
          },
        )
      );

      // Play button
      commands.spawn((
        ButtonBundle {
          style: Style { align_items: AlignItems::Center, justify_content: JustifyContent::Center, width: Val::Percent(10.0), height: Val::Percent(10.0), ..default() },
          background_color: Color::CRIMSON.into(),
          ..default()
        },
        ButtonAction::Play,
      )).with_children(|commands| {
        commands.spawn(
          TextBundle::from_section(
            "Play",
            TextStyle { font_size: 50.0, color: Color::BLACK.into(), ..default() },
          )
        );
      });

      // Quit button
      commands.spawn((
        ButtonBundle {
          style: Style { align_items: AlignItems::Center, justify_content: JustifyContent::Center, width: Val::Percent(10.0), height: Val::Percent(10.0), ..default() },
          background_color: Color::CRIMSON.into(),
          ..default()
        },
        ButtonAction::Quit,
      )).with_children(|commands| {
        commands.spawn(
          TextBundle::from_section(
            "Quit",
            TextStyle { font_size: 50.0, color: Color::BLACK.into(), ..default() },
          )
        );
      });
    });
}

fn despawn_menu(
  mut commands: Commands,
  menu: Query<Entity, With<MenuScreen>>,
) {
  commands.entity(menu.single()).despawn_recursive();
}

fn button_actions(
  interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
  mut game_state: ResMut<NextState<GameState>>,
  mut app_exit_events: EventWriter<AppExit>,
) {
  for (interaction, button_action) in &interaction_query {
    if *interaction != Interaction::Pressed {
      continue;
    }

    match button_action {
      ButtonAction::Play => { game_state.set(GameState::Manage); }
      ButtonAction::Quit => { app_exit_events.send(AppExit); }
    }
  }
}
