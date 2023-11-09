use bevy::prelude::*;
use fg_game::{GameModuleDescriptor, NativeGameModule, SimulationState};

pub fn get_base_game_module() -> GameModuleDescriptor {
  GameModuleDescriptor::Native(NativeGameModule { register_systems })
}

fn register_systems(init: &mut Schedule, update: &mut Schedule) {
  init.add_systems(on_load);
  update.add_systems(some_system.run_if(in_state(SimulationState::Simulating)));
}

pub fn on_load(_commands: Commands) {
  // todo: register loading tasks, load assets, spawn entities
}

pub fn some_system(_qry: Query<Entity>) {}
