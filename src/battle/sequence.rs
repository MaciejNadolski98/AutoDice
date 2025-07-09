use bevy::prelude::*;
use bevy_defer::{AccessError, AsyncAccess, AsyncCommandsExtension, AsyncWorld};

use crate::camera::SwapBattleCamera;
use crate::constants::DICE_SIZE;
use crate::manage::plugin::{Coins, EnemyTeam, MyTeam, ShopRound};
use crate::states::GameState;
use crate::dice::{move_dices_to_rows, resolve_dices, roll_dices, Dice};
use crate::utils::*;

pub struct SequencePlugin;

impl Plugin for SequencePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Battle), |mut commands: Commands| {
        commands.spawn_task(flow);
      })
      .add_event_and_listen::<StartGame>()
      .add_event_and_listen::<StartRound>()
      .add_event_and_listen::<BeforeRollDices>()
      .add_event_and_listen::<BeforeResolveDices>();
  }
}

#[derive(Event, Clone, Copy, Debug)]
pub struct StartGame;

#[derive(Event, Clone, Copy, Debug)]
pub struct StartRound {
  #[allow(unused)]
  round: u32,
}


#[derive(Event, Clone, Copy, Debug)]
pub struct BeforeRollDices;

#[derive(Event, Clone, Copy, Debug)]
pub struct BeforeResolveDices;

async fn flow() -> Result<(), AccessError> {
  AsyncWorld.sleep(0.1).await; // Wait for the world to be ready
  move_dices_to_rows().await?;

  AsyncWorld.trigger_event(StartGame.wrap()).await?;
  let mut current_round = 1;
  loop {
    AsyncWorld.trigger_event(StartRound { round: current_round }.wrap()).await?;

    AsyncWorld.trigger_event(BeforeRollDices.wrap()).await?;
    roll_dices().await?;

    AsyncWorld.trigger_event(BeforeResolveDices.wrap()).await?;
    resolve_dices().await?;

    if let Some(won) = done().await? {
      if won {
        if AsyncWorld.resource::<ShopRound>().get(|round| **round)? == 4 {
          end_game("WON!").await?;
        } else {
          AsyncWorld.resource::<ShopRound>().get_mut(|round| **round += 1)?;
          AsyncWorld.resource::<Coins>().get_mut(|coins| **coins += 5)?;
          AsyncWorld.set_state(GameState::Manage)?;
        }
      } else {
        end_game("LOST").await?;
      }
      return Ok(())
    }

    current_round += 1;
    AsyncWorld.send_event(SwapBattleCamera)?;
  }
}

async fn done() -> Result<Option<bool>, AccessError> {
  let dices = AsyncWorld.query::<&Dice>();
  let mut dices_1_left = false;
  let mut dices_2_left = false;
  dices.for_each(|dice| if dice.id().team_id == 0 { dices_1_left = true });
  dices.for_each(|dice| if dice.id().team_id == 1 { dices_2_left = true });
  
  if !dices_2_left {
    Ok(Some(true))
  } else if !dices_1_left {
    Ok(Some(false))
  } else {
    Ok(None)
  }
}

async fn end_game(ending_text: &'static str) -> Result<(), AccessError> {
  let end_screen = AsyncWorld.spawn_bundle((
    Node {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..default()
    },
    BackgroundColor(Color::BLACK),
  )).id();

  AsyncWorld.spawn_bundle((
    ChildOf(end_screen),
    Text::new(ending_text),
    TextFont {
      font_size: DICE_SIZE,
      ..default()
    },
    TextColor(Color::WHITE),
  ));

  AsyncWorld.run_system_cached(clean_up_game)?;

  AsyncWorld.sleep(3.0).await;

  AsyncWorld.entity(end_screen).despawn();

  AsyncWorld.set_state(GameState::Menu)?;
  Ok(())
}

pub fn clean_up_game(
  my_team: Single<Entity, With<MyTeam>>,
  enemy_team: Single<Entity, With<EnemyTeam>>,
  dices: Query<Entity, With<Dice>>,
  mut commands: Commands,
) {
  for dice in dices {
    commands.entity(dice).despawn();
  }
  commands.entity(*my_team).despawn();
  commands.entity(*enemy_team).despawn();
}
