use bevy::prelude::*;

use crate::Loading;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SimulationState {
  #[default]
  Disabled,
  Ready,
  Loading,
  Simulating,
  Unloading,
}

#[derive(Event, Debug)]
pub enum GameControlCommand {
  Initialize,
  NewGame,
  // JoinGame(PlayerDescriptor),
  // LeaveGame(PlayerDescriptor),
}

pub fn process_game_control_commands(
  mut cmds: EventReader<GameControlCommand>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
  current_state: Res<State<SimulationState>>,
) {
  for cmd in cmds.read() {
    match ((*current_state).get(), cmd) {
      (SimulationState::Disabled, GameControlCommand::Initialize) => {
        // set state so exclusive system to register module runs
        next_sim_state.set(SimulationState::Ready);
      }
      (SimulationState::Ready, GameControlCommand::NewGame) => {
        // todo: implement game modes

        // note: loading will be done hierarchically and in two passes
        // todo: add a resource where we can register loading tasks

        // signal to all systems to start loading whatever is required for new game
        next_sim_state.set(SimulationState::Loading);
      }
      _ => {
        unimplemented!()
      }
    }
  }
}

pub fn wait_until_loading_complete(
  qry: Query<Entity, With<Loading>>,
  mut next_state: ResMut<NextState<SimulationState>>,
) {
  if qry.is_empty() {
    next_state.set(SimulationState::Simulating);
  }
}
