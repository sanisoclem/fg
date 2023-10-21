use bevy::prelude::*;

pub trait GameEngineExtensions {
  fn add_game_engine(&mut self) -> &mut Self;
}

impl GameEngineExtensions for App {
  fn add_game_engine(&mut self) -> &mut Self {
    self
      .add_state::<SimulationState>()
      .init_resource::<ModuleManager>()
      .init_resource::<GameSession>()
      .add_event::<GameControlCommand>()
      .add_systems(Update, process_game_control_commands)
  }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SimulationState {
  #[default]
  Disabled,
  Ready,
  Loading,
  Simulating,
}

#[derive(Event)]
pub enum GameControlCommand {
  Reset,
  NewGame(GameModeDescriptor),
  // JoinGame(PlayerDescriptor),
  // LeaveGame(PlayerDescriptor),
  // ExitGame,
  // Teardown, // destroys the session
}

#[derive(Default, Resource)]
pub struct ModuleManager {
  modules: Vec<GameModuleDescriptor>,
}

impl ModuleManager {
  pub fn clear(&mut self) -> &mut Self {
    self.modules.clear();
    self
  }

  pub fn register(&mut self, module: GameModuleDescriptor) -> &mut Self {
    self.modules.push(module);
    self
  }

  pub fn run_init(&self, session: &mut GameSession) {
    for module in self.modules.iter() {
      match module {
        GameModuleDescriptor::Native(native_mod) => {
          (native_mod.on_init)(session);
        }
        _ => {
          unimplemented!()
        }
      }
    }
  }
}

#[derive(Default, Resource)]
pub struct GameSession {
  modes: Vec<GameModeDescriptor>,
}

impl GameSession {
  pub fn reset(&mut self) {
    self.modes.clear();
  }
  pub fn get_modes(&self) -> &[GameModeDescriptor] {
    &self.modes
  }

  pub fn add_mode(&mut self, mode: GameModeDescriptor) {
    self.modes.push(mode);
  }
}

#[derive(Clone)]
pub enum GameModuleDescriptor {
  Native(NativeGameModule),
  Script(ScriptGameModule),
}

#[derive(Clone)]
pub struct NativeGameModule {
  pub on_init: fn(&mut GameSession) -> (),
}

#[derive(Clone)]
pub struct ScriptGameModule;

#[derive(Clone)]
pub struct GameModeDescriptor;

pub struct PlayerDescriptor;

#[derive(Component)]
pub struct GameSessionComponent;

fn process_game_control_commands(
  mut commands: Commands,
  mut cmds: EventReader<GameControlCommand>,
  mut session: ResMut<GameSession>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,

  all_session_entities: Query<Entity, With<GameSessionComponent>>,
  mod_mgr: Res<ModuleManager>,
) {
  for cmd in cmds.iter() {
    match cmd {
      GameControlCommand::Reset => {
        // despawn all entities if any
        for entity in &all_session_entities {
          commands.entity(entity).despawn_recursive();
        }

        // clear all session dictionaries
        session.reset();

        // re-initialize all modules
        mod_mgr.run_init(&mut session);

        // signal that session is ready
        next_sim_state.set(SimulationState::Ready);
      },
      GameControlCommand::NewGame(mode) => {

      },
      _ => {}
    }
  }
}
