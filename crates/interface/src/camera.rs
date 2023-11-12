use bevy::prelude::*;

pub fn setup_camera(mut cmd: Commands) {
  cmd.spawn(Camera3dBundle {
    transform: Transform::from_xyz(0.0, 100.0, -2.0).looking_at(Vec3::ZERO, Vec3::Y),
    camera: Camera {
      hdr: true, // 1. HDR is required for bloom
      order: 0,
      ..default()
    },
    ..default()
  });
}

pub fn move_camera(_qry: Query<Entity>) {}