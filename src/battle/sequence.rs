use bevy::{ecs::system::SystemId, prelude::*};

use crate::states::GameState;

#[derive(Resource, Default)]
pub struct SystemStack {
  pub stack: Vec<SystemId>,
}

pub struct SequencePlugin;

impl Plugin for SequencePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<SystemStack>()
      .add_systems(Update, main_loop.run_if(in_state(GameState::Battle)))
      .add_systems(OnEnter(GameState::Battle), initialize_stack)
      .add_observer(add_to_stack);
  }
}

fn main_loop(
  system_stack: Res<SystemStack>,
  mut commands: Commands,
) {
  let Some(system_id) = system_stack.stack.last() else {
    return;
  };
  commands.run_system(*system_id);
}

#[derive(Event)]
struct AddToStack {
  system: SystemId,
}

fn add_to_stack(
  trigger: Trigger<AddToStack>,
  mut system_stack: ResMut<SystemStack>,
) {
  system_stack.stack.push(trigger.system);
}

#[derive(Resource, Default)]
enum FlowState {
  #[default]
  Init,
  RollingDices,
  ResolvingDices,
  RoundEnd,
}

fn flow(
  mut state: Local<FlowState>, 
  mut round: Local<u32>
){
  match *state {
    FlowState::Init => {
      info!("flow: Init");
      *state = FlowState::RollingDices;
      *round = 1;
    },
    FlowState::RollingDices => {
      info!("flow: RollingDices");
      *state = FlowState::ResolvingDices;
    },
    FlowState::ResolvingDices => {
      info!("flow: ResolvingDices");
      *state = FlowState::RoundEnd;
    },
    FlowState::RoundEnd => {
      info!("flow: RoundEnd, next round: {}", *round + 1);
      *round += 1;
      *state = FlowState::RollingDices;
    },
  }
}

fn initialize_stack(
  mut commands: Commands,
) {
  commands.register_system_and_add_to_stack(flow);
}

trait RegisterAndAddToStack {
  fn register_system_and_add_to_stack<M>(&mut self, system: impl IntoSystem<(), (), M> + 'static);
}

impl<'w, 's> RegisterAndAddToStack for Commands<'w, 's> {
  fn register_system_and_add_to_stack<M>(
    &mut self,
    system: impl IntoSystem<(), (), M> + 'static,
  ) {
    let system_id = self.register_system(system);
    self.trigger(AddToStack {
      system: system_id,
    });
  }
}
