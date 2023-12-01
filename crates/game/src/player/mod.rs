use bevy::{prelude::*, utils::HashMap};

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub struct PlayerId(u16);

pub struct PlayerDescriptor {
  pub id: PlayerId,
  pub name: String,
}

#[derive(Default, Resource)]
pub struct PlayerRegistry {
  players: HashMap<PlayerId, PlayerDescriptor>,
}

impl PlayerRegistry {
  pub fn join(&mut self, player: PlayerDescriptor) {
    self.players.insert(player.id, player);
  }
  pub fn leave(&mut self, player_id: &PlayerId) {
    self.players.remove(player_id);
  }
}
