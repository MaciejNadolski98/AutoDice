use bevy::prelude::*;

use crate::Money;

pub struct GameUI;

impl Plugin for GameUI {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_game_ui)
      .add_systems(Update, update_money_ui);
  }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MoneyText;

fn spawn_game_ui(
  mut commands: Commands,
) {
  commands.spawn((
    NodeBundle {
      style: Style {
        width: Val::Percent(100.0),
        height: Val::Percent(10.0),
        align_items: AlignItems::Center,
        padding: UiRect { left: Val::Percent(2.0), ..default() },
        ..default()
      },
      background_color: Color::BLUE.into(),
      ..default()
    },
    Name::new("UI Root"),
  ))
  .with_children(|commands| {
    commands.spawn((
      TextBundle {
        text: Text::from_section("Money", TextStyle { font_size: 48.0, ..default() }),
        ..default()
      },
      MoneyText,
      Name::new("Money"),
    ));
  });
}

fn update_money_ui(
  money_value: Res<Money>,
  mut money_ui: Query<&mut Text, With<MoneyText>>,
) {
  for mut text in &mut money_ui {
    text.sections[0].value = format!("Money: ${:?}", money_value.0);
  }
}
