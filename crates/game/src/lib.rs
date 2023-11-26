use bevy::prelude::*;

mod components;
mod mods;
mod state;
pub mod player;
pub mod blueprints;
pub mod build_area;

pub use components::*;
pub use mods::*;
pub use state::*;

pub trait GameEngineExtensions {
  fn add_game_engine(&mut self) -> &mut Self;
}

impl GameEngineExtensions for App {
  fn add_game_engine(&mut self) -> &mut Self {
    self
      .init_resource::<ModManager>()
      .init_resource::<blueprints::BlueprintRegistry>()
      .add_state::<SimulationState>()
      .add_event::<GameControlCommand>()
      .add_systems(Update, process_game_control_commands)
      .add_systems(OnExit(SimulationState::Disabled), register_mods)
      .add_systems(
        OnEnter(SimulationState::Loading),
        (run_mod_startup, wait_until_loading_complete).chain(),
      )
      .add_systems(
        Update,
        run_mod_update.run_if(in_state(SimulationState::Simulating)),
      )
  }
}
