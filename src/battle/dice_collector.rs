use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::{constants::DICE_SIZE, dice::RespawnDicesEvent, states::GameState, dice::dice_instance::Dice};


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
impl DiceCollector {
  fn tick(&mut self, delta: Duration) {
    self.min_throw_timer.tick(delta);
    self.max_throw_timer.tick(delta);
    self.stationary_timer.tick(delta);
    self.collecting_timer.tick(delta);
    self.presenting_timer.tick(delta);
  }
  fn set_inactive(&mut self) {
    self.stage = BattleStage::Inactive;
  }

  fn set_pending(&mut self) {
    self.stage = BattleStage::Pending;
  }

  fn set_throwing(&mut self) {
    self.stage = BattleStage::Throwing;
    info!("Throwing state");
    self.min_throw_timer.reset();
    self.max_throw_timer.reset();
    self.stationary_transforms.clear();
    self.target_transforms.clear();
  }

  fn set_stationary(&mut self) {
    self.stage = BattleStage::Stationary;
    self.stationary_timer.reset();
  }

  fn set_collecting(&mut self) {
    self.stage = BattleStage::Collecting;
    self.collecting_timer.reset();
  }

  fn set_presenting(&mut self) {
    self.stage = BattleStage::Presenting;
    self.presenting_timer.reset();
  }

  fn set_executing(&mut self) {
    self.stage = BattleStage::Executing;
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
  dice_collector.set_throwing();
}
// TODO: Split into into simpler functions
fn animate(
  mut dice_collector: ResMut<DiceCollector>,
  mut query: Query<(&Dice, &mut Transform, &mut LinearVelocity, &mut AngularVelocity, &mut RigidBody)>,
  time: Res<Time>,
  mut commands: Commands){
    dice_collector.tick(time.delta());
    if dice_collector.stage == BattleStage::Throwing{
      // dice_collector.min_throw_timer.tick(time.delta());
      // dice_collector.max_throw_timer.tick(time.delta());
      if dice_collector.min_throw_timer.finished() && are_dices_stationary(query){
        info!("Stationary state");
        dice_collector.set_stationary();

      }
      if dice_collector.max_throw_timer.finished(){
        info!("Max throw state time elapsed, stopping dices");
        dice_collector.set_stationary();
      }
    }
    else if dice_collector.stage == BattleStage::Stationary{
      // dice_collector.stationary_timer.tick(time.delta());

      if dice_collector.stationary_transforms.len() == 0{
        for (i, (d, t, _, _, mut rb)) in query.iter_mut().enumerate() {
          info!("{:?}", Mat3::from_quat(t.rotation));
          dice_collector.stationary_transforms.push(t.clone());
          let m = get_discrete_dice_orientation(*t);
          let mut target_transform = t.clone();
          target_transform.rotation = Quat::from_mat3(&m);
          let mut z = DICE_SIZE as f32;
          if d == &Dice::Blue{
            z = -z;
          }
          target_transform.translation = Vec3::new((i as f32 - 5.0) * DICE_SIZE * 2.0, DICE_SIZE / 2.0, z);
          dice_collector.target_transforms.push(target_transform);
          *rb = RigidBody::Static;          
        }
      }

      if dice_collector.stationary_timer.finished(){
        info!("Collecting state");
        dice_collector.set_collecting();
      }
    }
    else if dice_collector.stage == BattleStage::Collecting{
      // dice_collector.collecting_timer.tick(time.delta());
      if dice_collector.collecting_timer.finished(){
        info!("Presenting state");
        dice_collector.set_presenting();
      }
      else{
        let t = dice_collector.collecting_timer.elapsed().as_secs_f32()/dice_collector.collecting_timer.duration().as_secs_f32();
        for (i, (_, mut transform, _, _, _)) in query.iter_mut().enumerate() {
          transform.translation = dice_collector.stationary_transforms[i].translation.lerp(dice_collector.target_transforms[i].translation, t);
          transform.rotation = dice_collector.stationary_transforms[i].rotation.slerp(dice_collector.target_transforms[i].rotation, t);
          // linear_velocity.0 = Vec3::ZERO;
          // angular_velocity.0 = Vec3::ZERO;
        }
      }
    }
    else if dice_collector.stage == BattleStage::Presenting{
      // dice_collector.presenting_timer.tick(time.delta());
      if dice_collector.collecting_timer.finished(){
        info!("Executing state");
        dice_collector.set_executing();
      }
      else{
        
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

fn are_dices_stationary(
  query: Query<(&Dice, &mut Transform, &mut LinearVelocity, &mut AngularVelocity, &mut RigidBody)>
) -> bool {
  for (_, _, velocity, _, _) in &query {
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
