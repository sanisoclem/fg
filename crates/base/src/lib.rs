use bevy::prelude::*;
use fg_game::{GameModuleDescriptor, NativeGameModule};

pub fn get_base_game_module() -> GameModuleDescriptor {
  GameModuleDescriptor::Native(NativeGameModule {
    register_startup,
    register_update,
  })
}

fn register_startup(sched: &mut Schedule) {
  sched.add_systems(on_load);
}

fn register_update(sched: &mut Schedule) {
  sched.add_systems(some_system);
}

pub fn on_load(_commands: Commands) {
  // todo: register loading tasks, load assets, spawn entities
}

pub fn some_system(_qry: Query<Entity>) {}
