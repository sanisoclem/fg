use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

#[derive(Default, Resource)]
pub struct ModManager {
  modules: Vec<GameModuleDescriptor>,
}

impl ModManager {
  pub fn clear(&mut self) -> &mut Self {
    self.modules.clear();
    self
  }

  pub fn register(&mut self, module: GameModuleDescriptor) -> &mut Self {
    self.modules.push(module);
    self
  }

  pub fn build_startup_schedule(&self, label: impl ScheduleLabel) -> Schedule {
    let mut sched = Schedule::new(label);
    for module in self.modules.iter() {
      module.register_startup(&mut sched);
    }
    sched
  }
  pub fn build_update_schedule(&self, label: impl ScheduleLabel) -> Schedule {
    let mut sched = Schedule::new(label);
    for module in self.modules.iter() {
      module.register_update(&mut sched);
    }
    sched
  }
}

#[derive(Clone, PartialEq)]
pub enum GameModuleDescriptor {
  Native(NativeGameModule),
  Script(ScriptGameModule),
}

impl GameModuleDescriptor {
  pub fn register_startup(&self, sched: &mut Schedule) {
    if let &GameModuleDescriptor::Native(native_mod) = &self {
      (native_mod.register_startup)(sched);
    }
  }
  pub fn register_update(&self, sched: &mut Schedule) {
    if let &GameModuleDescriptor::Native(native_mod) = &self {
      (native_mod.register_update)(sched);
    }
  }
}

#[derive(Clone, PartialEq)]
pub struct NativeGameModule {
  pub register_startup: fn(sched: &mut Schedule) -> (),
  pub register_update: fn(sched: &mut Schedule) -> (),
}

#[derive(Clone, PartialEq)]
pub struct ScriptGameModule;
