use bevy::prelude::*;
use utils::despawn_screen;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub trait SplashExtensions {
  fn add_splash_screen<T: States + Copy>(&mut self, show_on_state: T, next_state: T) -> &mut Self;
}

impl SplashExtensions for App {
  fn add_splash_screen<T: States + Copy>(&mut self, show_on_state: T, next_state: T) -> &mut Self {
    self
      .insert_resource(SplashNextState(next_state))
      .add_systems(OnEnter(show_on_state), splash_setup)
      .add_systems(Update, countdown::<T>.run_if(in_state(show_on_state)))
      .add_systems(OnExit(show_on_state), despawn_screen::<OnSplashScreen>)
  }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

#[derive(Resource)]
struct SplashNextState<T>(T);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let icon = asset_server.load("splash.png");
  let font = asset_server.load("fonts/FiraSans-Bold.ttf");

  commands.spawn((Camera2dBundle::default(), OnSplashScreen));
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
      OnSplashScreen,
    ))
    .with_children(|parent| {
      parent.spawn(ImageBundle {
        style: Style {
          width: Val::Px(200.0),
          ..default()
        },
        image: UiImage::new(icon),
        ..default()
      });
      parent.spawn(
        TextBundle::from_section(
          "Bevy Game Jam #3",
          TextStyle {
            font: font.clone(),
            font_size: 80.0,
            color: TEXT_COLOR,
          },
        )
        .with_style(Style {
          margin: UiRect::all(Val::Px(50.0)),
          ..default()
        }),
      );
    });

  commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn countdown<T: States>(
  mut timer: ResMut<SplashTimer>,
  mut game_state: ResMut<NextState<T>>,
  next_state: Res<SplashNextState<T>>,
  time: Res<Time>,
) {
  if timer.tick(time.delta()).finished() {
    game_state.set(next_state.0.clone());
  }
}
