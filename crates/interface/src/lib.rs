use bevy::prelude::*;
use camera::*;
use fg_game::{GameModuleDescriptor, NativeGameModule};

mod camera;

pub fn get_ui_game_module() -> GameModuleDescriptor {
  GameModuleDescriptor::Native(NativeGameModule {
    register_startup,
    register_update,
  })
}

fn register_startup(sched: &mut Schedule) {
  // how can we associate the camera with a user??
  sched.add_systems(setup_camera);
}

fn register_update(sched: &mut Schedule) {
  sched.add_systems(move_camera);
}
