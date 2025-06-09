use bevy::prelude::*;
use bevy_defer::{AccessError, AsyncCommandsExtension, AsyncWorld};

use crate::camera::SwapBattleCamera;
use crate::states::GameState;
use crate::dice::{resolve_dices, roll_dices};

pub struct SequencePlugin;

impl Plugin for SequencePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Battle), |mut commands: Commands| {
        commands.spawn_task(|| flow());
      });
  }
}

async fn flow() -> Result<(), AccessError> {
  let mut current_round = 1;
  loop {
    info!("Round {}", current_round);

    roll_dices().await?;

    resolve_dices().await?;

    current_round += 1;
    AsyncWorld.send_event(SwapBattleCamera)?;
  }
}
