use bevy::prelude::*;
use fg_base::get_base_game_module;
use fg_game::{GameControlCommand, GameEngineExtensions, ModManager, SimulationState};
use loading::*;
use camera::CameraExtensions;

mod camera;
mod input;
mod loading;

#[derive(Resource)]
struct GameNextState<T>(T);
pub trait GameExtensions {
  fn add_game<T: States + Copy>(&mut self, game_state: T) -> &mut Self;
}

impl GameExtensions for App {
  fn add_game<T: States + Copy>(&mut self, game_state: T) -> &mut Self {
    self
      .add_game_engine()
      .add_game_loading_screen()
      .add_game_camera()
      .add_systems(OnEnter(game_state), setup_base_game)
      .add_systems(OnEnter(SimulationState::Ready), start_base_game)
  }
}

fn setup_base_game(mut mod_mgr: ResMut<ModManager>, mut cmds: EventWriter<GameControlCommand>) {
  // hard code the base module
  mod_mgr.clear().register(get_base_game_module());
  // initialize the session
  cmds.send(GameControlCommand::Initialize);
}

fn start_base_game(mut cmds: EventWriter<GameControlCommand>) {
  // start the first game mode
  cmds.send(GameControlCommand::NewGame);
}

// TODO:
//  - wait for GameState::Started send GameControlCommand::JoinGame

fn join_game(mut cmd: Commands) {}
