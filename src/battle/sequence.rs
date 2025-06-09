use bevy::prelude::*;
use bevy_defer::{AccessError, AsyncCommandsExtension, AsyncWorld};

use crate::camera::SwapBattleCamera;
use crate::states::GameState;
use crate::dice::{resolve_dices, roll_dices, Dice};
use crate::utils::*;

pub struct SequencePlugin;

impl Plugin for SequencePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Battle), |mut commands: Commands| {
        commands.spawn_task(|| flow());
      })
      .add_event_and_listen::<StartRound>()
      .add_event_and_listen::<BeforeRollDices>()
      .add_event_and_listen::<BeforeResolveDices>();
  }
}

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
  let mut current_round = 1;
  loop {
    AsyncWorld.trigger_event(StartRound::new(StartRound { round: current_round })).await?;

    AsyncWorld.trigger_event(BeforeRollDices::new(BeforeRollDices)).await?;
    roll_dices().await?;

    AsyncWorld.trigger_event(BeforeResolveDices::new(BeforeResolveDices)).await?;
    resolve_dices().await?;

    if done().await? {
      info!("Battle finished");
      AsyncWorld.set_state(GameState::Manage)?;
      return Ok(())
    }

    current_round += 1;
    AsyncWorld.send_event(SwapBattleCamera)?;
  }
}

async fn done() -> Result<bool, AccessError> {
  let dices = AsyncWorld.query::<&Dice>();
  let mut dices_1_left = false;
  let mut dices_2_left = false;
  dices.for_each(|dice| if dice.id().team_id == 0 { dices_1_left = true });
  dices.for_each(|dice| if dice.id().team_id == 1 { dices_2_left = true });
  
  Ok(!(dices_1_left && dices_2_left))
}
