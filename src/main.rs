use std::time::Duration;

use bevy::prelude::*;
use fg_game::GameEngineExtensions;
use menu::MenuExtensions;
use splash::SplashExtensions;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
  #[default]
  Splash,
  Menu,
  Game,
}

mod game;
mod loading;
mod menu;
mod splash;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    .add_splash_screen(AppState::Splash, AppState::Menu)
    .add_main_menu(AppState::Menu, AppState::Game)
    .add_game_engine()
    .run();
}
