use std::default;

use bevy::prelude::*;

#[derive(Resource)]
struct GameNextState<T>(T);
pub trait GameExtensions {
  fn add_game<T: States>(&mut self, game_state: T, exit_state: T) -> &mut Self;
}

impl GameExtensions for App {
  fn add_game<T: States>(&mut self, _game_state: T, _exit_state: T) -> &mut Self {
    self
      .add_state::<GameState>()
      .init_resource::<ModuleManager>()
      .init_resource::<GameSession>()
      .add_event::<GameControlCommand>()
  }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
  #[default]
  Disabled, // no game systems running
  Initialized, // modules have been initialized
  Loading,     //
  Started,     // game is ongoing
}

#[derive(Event)]
pub enum GameControlCommand {
  Init, // initializes the game session using the registered modules
  NewGame(GameModeDescriptor),
  JoinGame(PlayerDescriptor),
  LeaveGame(PlayerDescriptor),
  ExitGame,
  Teardown, // destroys the session
}

#[derive(Default, Resource)]
pub struct ModuleManager;

impl ModuleManager {
  pub fn clear(&mut self) {
    unimplemented!()
  }
  pub fn register(&mut self, _module: GameModuleDescriptor) {
    unimplemented!()
  }
  pub fn create_session(&self) -> GameSession {
    unimplemented!()
  }
}

#[derive(Default, Resource)]
pub struct GameSession;

impl GameSession {
  pub fn get_modes(&self) -> &[GameModeDescriptor] {
    unimplemented!()
  }
  pub fn new_game(&self) -> GameInstance {
    unimplemented!()
  }
}

#[derive(Default, Resource)]
pub struct GameInstance;

impl GameInstance {
  pub fn join(&mut self, _player: PlayerDescriptor) {
    unimplemented!()
  }
}

pub struct GameModuleDescriptor;
pub struct GameModeDescriptor;
pub struct PlayerDescriptor;

// // TODO: traits are placeholders, replace with structs
// pub trait GameSessionManager {
//   fn clear_modules(&mut self);
//   fn register_game_module(&mut self, module: GameModuleDescriptor);
//   fn create_session(self) -> dyn GameSession;
// }

// pub trait GameSession {
//   fn get_modes(&self) -> [GameModeDescriptor];
//   fn new_game(self, mode: &GameModeDescriptor) -> dyn GameInstance;
//   fn load_game(self, data: SaveData) -> dyn GameInstance;
// }

// pub trait GameInstance {
//   fn join(&mut self, player: PlayerDescriptor);
// }

// // pub struct SessionSettings {
// //   pub name: String,
// //   pub seed: [u8; 16],
// //   pub engine_version: u32,
// // }

// // pub struct GameSave {
// //   pub session_id: String,
// //   pub settings: SessionSettings,
// //   pub modules: Vec<GameModuleId>
// // }

// // pub struct GameModuleId(String, u32);
// // pub struct GameModuleMetadata {
// //   pub id: GameModuleId,
// //   pub name: String
// // }
