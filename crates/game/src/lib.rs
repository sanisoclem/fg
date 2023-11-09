use bevy::prelude::*;

pub trait GameEngineExtensions {
  fn add_game_engine(&mut self) -> &mut Self;
}

impl GameEngineExtensions for App {
  fn add_game_engine(&mut self) -> &mut Self {
    self
      .add_state::<SimulationState>()
      .init_resource::<ModuleManager>()
      .init_resource::<ModuleRunner>()
      .add_event::<GameControlCommand>()
      .add_systems(Update, process_game_control_commands)
      .add_systems(OnEnter(SimulationState::Loading), run_mod_startup)
      .add_systems(
        Update,
        run_mod_update.run_if(in_state(SimulationState::Simulating)),
      )
  }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SimulationState {
  #[default]
  Disabled,
  Ready,
  Loading,
  Simulating,
  Unloading,
}

#[derive(Event, Debug)]
pub enum GameControlCommand {
  Initialize,
  NewGame,
  // JoinGame(PlayerDescriptor),
  // LeaveGame(PlayerDescriptor),
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
}

#[derive(Default, Resource)]
pub struct ModuleRunner {
  update_systems: Schedule,
  startup_systems: Schedule,
}

impl ModuleRunner {
  pub fn load(&mut self, mods: &ModuleManager) {
    let mut startup_sched = Schedule::default();
    let mut update_sched = Schedule::default();
    for module in mods.modules.iter() {
      module.register_systems(&mut startup_sched, &mut update_sched);
    }

    self.update_systems = update_sched;
    self.startup_systems = startup_sched;
  }
}

#[derive(Clone, PartialEq)]
pub enum GameModuleDescriptor {
  Native(NativeGameModule),
  Script(ScriptGameModule),
}

impl GameModuleDescriptor {
  pub fn register_systems(&self, startup_sched: &mut Schedule, update_sched: &mut Schedule) {
    if let &GameModuleDescriptor::Native(native_mod) = &self {
      (native_mod.register_systems)(startup_sched, update_sched);
    }
  }
}

#[derive(Clone, PartialEq)]
pub struct NativeGameModule {
  pub register_systems: fn(startup_sched: &mut Schedule, update_sched: &mut Schedule) -> (),
}

#[derive(Clone, PartialEq)]
pub struct ScriptGameModule;

fn process_game_control_commands(
  mut cmds: EventReader<GameControlCommand>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
  mut module_runner: ResMut<ModuleRunner>,
  current_state: Res<State<SimulationState>>,
  mod_mgr: Res<ModuleManager>,
) {
  for cmd in cmds.read() {
    match ((*current_state).get(), cmd) {
      (SimulationState::Disabled, GameControlCommand::Initialize) => {
        // initialize all modules
        module_runner.load(&mod_mgr);

        // note: initialization should be fast and only runs in single frame
        // preferrably, minimal io like registering systems and resources to load later

        // signal all modules are ready
        next_sim_state.set(SimulationState::Ready);
      }
      (SimulationState::Ready, GameControlCommand::NewGame) => {
        // todo: implement game modes

        // note: loading will be done hierarchically and in two passes
        // todo: add a resource where we can register loading tasks

        // signal to all systems to start loading whatever is required for new game
        next_sim_state.set(SimulationState::Loading);
      }
      _ => {
        unimplemented!()
      }
    }
  }
}

fn run_mod_startup(world: &mut World) {
  let mut runner = world.resource_mut::<ModuleRunner>();
  // where can I store the schedule? cannot store in world without cloning (but schedule does not clone?)
  //runner.startup_systems.run(&mut world);
}

fn run_mod_update(world: &mut World) {
  let mut runner = world.resource_mut::<ModuleRunner>();
  //runner.update_systems.run(world);
}
