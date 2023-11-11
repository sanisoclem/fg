use bevy::{prelude::*, ecs::schedule::ScheduleLabel};

pub trait GameEngineExtensions {
  fn add_game_engine(&mut self) -> &mut Self;
}

impl GameEngineExtensions for App {
  fn add_game_engine(&mut self) -> &mut Self {
    self
      .add_state::<SimulationState>()
      .init_resource::<ModuleManager>()
      .add_event::<GameControlCommand>()
      .add_systems(Update, process_game_control_commands)
      .add_systems(OnEnter(SimulationState::Ready), register_mods)
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

  pub fn build_startup_schedule(&self) -> Schedule {
    let mut sched = Schedule::new(ModStartup);
    for module in self.modules.iter() {
      module.register_startup(&mut sched);
    }
    sched
  }
  pub fn build_update_schedule(&self) -> Schedule {
    let mut sched = Schedule::new(ModUpdate);
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

fn process_game_control_commands(
  mut cmds: EventReader<GameControlCommand>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
  current_state: Res<State<SimulationState>>,
  mod_mgr: Res<ModuleManager>,
) {
  for cmd in cmds.read() {
    match ((*current_state).get(), cmd) {
      (SimulationState::Disabled, GameControlCommand::Initialize) => {
        // set state so exclusive system to register module runs
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
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct ModStartup;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct ModUpdate;

fn register_mods(world: &mut World) {
  let startup_sched;
  let update_sched;
  {
    let mgr = world.resource::<ModuleManager>();
    startup_sched = mgr.build_startup_schedule();
    update_sched = mgr.build_update_schedule();
  }

  world.add_schedule(startup_sched);
  world.add_schedule(update_sched);
}


fn run_mod_startup(world: &mut World) {
  world.run_schedule(ModStartup);
}

fn run_mod_update(world: &mut World) {
  world.run_schedule(ModUpdate);
}
