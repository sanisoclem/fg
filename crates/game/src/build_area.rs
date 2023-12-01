use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct BuildDirector {
  pub layout: BuildLayout,
  pub build_mode: bool
}

pub enum BuildLayout {
  Grid(f32, f32)
}


pub fn draw_build_grid(director: Res<BuildDirector>) {
  if director.build_mode {
    // TODO: draw grid lines
  }
}