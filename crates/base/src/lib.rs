use bevy::prelude::*;
use fg_game::{GameModeDescriptor, GameModuleDescriptor, GameSession, NativeGameModule};

pub fn get_base_game_module() -> GameModuleDescriptor {
  GameModuleDescriptor::Native(NativeGameModule {
    on_init,
    on_new_game,
  })
}

pub fn on_init(session: &mut GameSession) {
  session.add_mode(GameModeDescriptor {
    id: 0,
    name: "Test".to_owned(),
  })
}

pub fn on_new_game(
  _mode: &GameModeDescriptor,
  _session: &mut GameSession,
  _commands: &mut Commands,
) {
}
