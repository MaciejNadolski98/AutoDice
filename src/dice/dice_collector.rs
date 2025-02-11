use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::states::GameState;


use super::{dice_instance::Dice, RespawnDicesEvent};

#[derive(PartialEq)]
enum BattleStage {
  Inactive,
  Pending,
  Throwing,
  Stationary,
  Collecting,
  Presenting,
  Executing
}
#[derive(Resource)]
struct DiceCollector {
  stage: BattleStage,
  stationary_transforms: Vec<Transform>,
  target_transforms: Vec<Transform>,

  // Always allow dices to be in throwing state for at least this time
  min_throw_timer: Timer,
  // Always end throwing state after this time
  max_throw_timer: Timer,
  // Let the dices sit still for this time before collecting them
  stationary_timer: Timer,
  // Animate collecting for this time
  collecting_timer: Timer,
  // Present collected dices for this time
  presenting_timer: Timer
}

impl Default for DiceCollector {
  fn default() -> Self {
    DiceCollector {
      stage: BattleStage::Inactive,
      stationary_transforms: Vec::new(),
      target_transforms: Vec::new(),
      min_throw_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
      max_throw_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
      stationary_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
      collecting_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
      presenting_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
    }
  }
}

pub struct DiceCollectorPlugin;

impl Plugin for DiceCollectorPlugin {
  fn build(&self, app: &mut App) {
    app
    .init_resource::<DiceCollector>()
    .add_systems(Update, animate.run_if(in_state(GameState::Battle)))
    .add_systems(Update, set_throwing.run_if(on_event::<RespawnDicesEvent>()));
  }
}

fn set_throwing(mut dice_collector: ResMut<DiceCollector>){
  dice_collector.stage = BattleStage::Throwing;
  info!("Throwing state");
  dice_collector.min_throw_timer.reset();
  dice_collector.max_throw_timer.reset();
  dice_collector.stationary_transforms.clear();
  dice_collector.target_transforms.clear();
}

fn animate(
  mut dice_collector: ResMut<DiceCollector>,
  mut query: Query<(&Dice, &mut Transform, &mut LinearVelocity, &mut AngularVelocity)>,
  time: Res<Time>,){
    if dice_collector.stage == BattleStage::Throwing{
      dice_collector.min_throw_timer.tick(time.delta());
      dice_collector.max_throw_timer.tick(time.delta());
      if dice_collector.min_throw_timer.finished() && is_dice_stationary(query){
        info!("Stationary state");
        dice_collector.stage = BattleStage::Stationary;
        dice_collector.stationary_timer.reset();

      }
      if dice_collector.max_throw_timer.finished(){
        info!("Max throw state time elapsed, stopping dices");
        dice_collector.stage = BattleStage::Stationary;
        dice_collector.stationary_timer.reset();
      }
    }
    else if dice_collector.stage == BattleStage::Stationary{
      dice_collector.stationary_timer.tick(time.delta());

      if dice_collector.stationary_transforms.len() == 0{
        for (d, t, lv, av) in &query{
          info!("{:?}", Mat3::from_quat(t.rotation));
          dice_collector.stationary_transforms.push(t.clone());
          let m = get_discrete_dice_orientation(*t);
          let mut target_transform = t.clone();
          target_transform.rotation = Quat::from_mat3(&m);
          dice_collector.target_transforms.push(target_transform);
        }
      }

      for (i, (_, mut transform, mut linear_velocity, mut angular_velocity)) in query.iter_mut().enumerate() {
        transform.translation = dice_collector.stationary_transforms[i].translation.clone();
        // for some reason resetting rotation causes some weird behavior (some dice do a 90 or 180 degrees spin across 1 or 2 frames)
        // transform.rotation = dice_collector.stationary_transforms[i].rotation.clone();
        linear_velocity.0 = Vec3::ZERO;
        angular_velocity.0 = Vec3::ZERO;
      }

      if dice_collector.stationary_timer.finished(){
        info!("Collecting state");
        dice_collector.stage = BattleStage::Collecting;
        dice_collector.collecting_timer.reset();

      }
    }
    else if dice_collector.stage == BattleStage::Collecting{
      dice_collector.collecting_timer.tick(time.delta());
      if dice_collector.collecting_timer.finished(){
        info!("Executing state");
        dice_collector.stage = BattleStage::Presenting;
        dice_collector.presenting_timer.reset();
      }
      else{
        let t = dice_collector.collecting_timer.elapsed().as_secs_f32()/dice_collector.collecting_timer.duration().as_secs_f32();
        for (i, (_, mut transform, mut linear_velocity, mut angular_velocity)) in query.iter_mut().enumerate() {
          transform.translation = dice_collector.stationary_transforms[i].translation.lerp(dice_collector.target_transforms[i].translation, t);
          transform.rotation = dice_collector.stationary_transforms[i].rotation.slerp(dice_collector.target_transforms[i].rotation, t);
          linear_velocity.0 = Vec3::ZERO;
          angular_velocity.0 = Vec3::ZERO;
        }
      }
    }
    else if dice_collector.stage == BattleStage::Presenting{
      dice_collector.presenting_timer.tick(time.delta());
      if dice_collector.collecting_timer.finished(){
        info!("Executing state");
        dice_collector.stage = BattleStage::Executing;
        dice_collector.presenting_timer.reset();
      }
      else{
        for (i, (_, mut transform, mut linear_velocity, mut angular_velocity)) in query.iter_mut().enumerate() {
          transform.translation = dice_collector.target_transforms[i].translation.clone();
          // for some reason resetting rotation causes some weird behavior (some dice do a 90 or 180 degrees spin across 1 or 2 frames)
          // transform.rotation = dice_collector.stationary_transforms[i].rotation.clone();
          linear_velocity.0 = Vec3::ZERO;
          angular_velocity.0 = Vec3::ZERO;
        }
      }

      
    }
    else if dice_collector.stage == BattleStage::Executing{
      // info!("Executing state");
      // dice_collector.stage = BattleStage::Inactive;
    }
    else{
      // info!("Inactive state");
    }
  }

fn is_dice_stationary(
  query: Query<(&Dice, &mut Transform, &mut LinearVelocity, &mut AngularVelocity)>
) -> bool {
  if query.iter().count() == 0 {
    return false;
  }
  for (_, _, velocity, _) in &query {
    if velocity.length() > 0.5 {
      return false;
    }
  }
  return true;
}
// Calculates the closest valid right angle orientation matrix to a given transform.
// Right angle orientation matrix consists only of 0, 1 and -1 values.
fn get_discrete_dice_orientation(t: Transform) -> Mat3{
  
  let rotation_matrix = Mat3::from_quat(t.rotation);
  // info!("{:?}", rotation_matrix);
  let mut max_vert_index = 0;
  let mut max_vert_value: f32 = 0.0;
  let mut max_horizontal_index = 0;
  let mut max_horizontal_value: f32 = 0.0;
  // searching for the most vertical component (highest value in y row)
  for i in 0..3 {
    if rotation_matrix.row(1)[i].abs() > max_vert_value.abs() {
      max_vert_index = i;
      max_vert_value = rotation_matrix.row(1)[i];
    }
  }
  // searching for the most vertical component (highest value in x row). 
  // Must be different than max vertical index
  for i in 0..3 {
    if i == max_vert_index {
      continue;
    }
    if rotation_matrix.row(0)[i].abs() > max_horizontal_value.abs() {
      max_horizontal_index = i;
      max_horizontal_value = rotation_matrix.row(0)[i];
    }
  }
  let mut y_row = Vec3::ZERO;
  y_row[max_vert_index] = max_vert_value / max_vert_value.abs();
  let mut x_row = Vec3::ZERO;
  x_row[max_horizontal_index] = max_horizontal_value / max_horizontal_value.abs();
  let z_row = x_row.cross(y_row);
  return Mat3::from_cols(x_row, y_row, z_row).transpose();

}
