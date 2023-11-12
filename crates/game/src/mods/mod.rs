use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
pub use manager::*;

mod manager;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct ModStartup;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct ModUpdate;

pub fn register_mods(world: &mut World) {
  let startup_sched;
  let update_sched;
  {
    let mgr = world.resource::<ModManager>();
    startup_sched = mgr.build_startup_schedule(ModStartup);
    update_sched = mgr.build_update_schedule(ModUpdate);
  }

  world.add_schedule(startup_sched);
  world.add_schedule(update_sched);
}

pub fn run_mod_startup(world: &mut World) {
  world.run_schedule(ModStartup);
}

pub fn run_mod_update(world: &mut World) {
  world.run_schedule(ModUpdate);
}
