use bevy::prelude::*;
use fg_game::{Loading, SimulationState};
use utils::despawn_screen;

pub trait LoadingExtensions {
  fn add_game_loading_screen(&mut self) -> &mut Self;
}

impl LoadingExtensions for App {
  fn add_game_loading_screen(&mut self) -> &mut Self {
    self
      .add_systems(OnEnter(SimulationState::Loading), (loading_setup))
      .add_systems(
        Update,
        update_progress.run_if(in_state(SimulationState::Loading)),
      )
      .add_systems(
        OnExit(SimulationState::Loading),
        despawn_screen::<OnLoadingScreen>,
      )
  }
}

#[derive(Component)]
struct OnLoadingScreen;

fn loading_setup(mut commands: Commands) {
  commands.spawn((Camera2dBundle::default(), OnLoadingScreen));
  commands
    .spawn((
      NodeBundle {
        style: Style {
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          ..default()
        },
        ..default()
      },
      OnLoadingScreen,
    ))
    .with_children(|parent| {
      parent.spawn(
        TextBundle::from_section(
          "Loading...",
          TextStyle {
            font_size: 80.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
          },
        )
        .with_style(Style {
          margin: UiRect::all(Val::Px(50.0)),
          ..default()
        }),
      );
    });
}

fn update_progress(qry: Query<Entity, With<Loading>>) {
  let in_progress_count = qry.iter().count();
  // TODO: update progress indicator
}
