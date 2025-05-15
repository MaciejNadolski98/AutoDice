use bevy::{prelude::*, app::AppExit, ui::Interaction};
use crate::states::GameState;

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
    Name::new("Main Menu"),
  ))
    .with_children(|commands| {
      // Main menu text
      commands.spawn((
        Text("AutoChess game thingy".to_string()),
        TextFont {
            font_size: 80.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 0.0, 0.0)),
    ));

      // Play button
      commands.spawn((
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

      // Quit button
      commands.spawn((
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
      ButtonAction::Quit => { app_exit_events.send(AppExit::Success); }
    }
  }
}
