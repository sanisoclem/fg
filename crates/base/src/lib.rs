use fg_game::{GameModeDescriptor, GameModuleDescriptor, GameSession, NativeGameModule};

pub fn get_base_game_module() -> GameModuleDescriptor {
  GameModuleDescriptor::Native(NativeGameModule { on_init })
}

pub fn on_init(session: &mut GameSession) {
  session.add_mode(GameModeDescriptor)
}
