use bevy::prelude::*;

#[derive(Resource)]
struct GameNextState<T>(T);
pub trait GameExtensions {
  fn add_game<T: States + Copy>(&mut self, game_state: T, exit_state: T) -> &mut Self;
}

impl GameExtensions for App {
  fn add_game<T: States + Copy>(&mut self, game_state: T, _exit_state: T) -> &mut Self {
    self.add_systems(OnEnter(game_state), join_game)
  }
}

// TODO:
//  - wait for GameState::Started send GameControlCommand::JoinGame

fn join_game(mut cmd: Commands) {

}
