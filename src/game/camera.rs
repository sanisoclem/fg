use bevy::prelude::*;
use fg_game::SimulationState;
use utils::despawn_screen;

pub trait CameraExtensions {
  fn add_game_camera(&mut self) -> &mut Self;
}

impl CameraExtensions for App {
  fn add_game_camera(&mut self) -> &mut Self {
    self
      .add_systems(OnEnter(SimulationState::Simulating), (setup_camera))
      .add_systems(
        Update,
        update_camera.run_if(in_state(SimulationState::Simulating)),
      )
      .add_systems(
        OnExit(SimulationState::Simulating),
        despawn_screen::<PlayerCamera>,
      )
  }
}

#[derive(Component)]
struct PlayerCamera;

fn setup_camera(mut cmd: Commands) {
  cmd
    .spawn(
      Camera3dBundle {
        transform: Transform::from_xyz(0.0, 100.0, -2.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
          hdr: true, // 1. HDR is required for bloom
          order: 0,
          ..default()
        },
        ..default()
      },
    )
    .insert(PlayerCamera);
}

fn update_camera(qry: Query<(&mut Transform), With<PlayerCamera>>) {
  let camera_input = 
}
