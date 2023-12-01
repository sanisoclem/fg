use bevy::prelude::*;



#[derive(Event)]
pub enum PlayerInput {
  CameraUp,
  CameraDown,
  CameraLeft,
  CameraRight,
}

pub trait ControlMapper<T> {
  
}