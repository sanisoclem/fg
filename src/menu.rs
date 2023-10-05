use bevy::{app::AppExit, prelude::*};
use utils::despawn_screen;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
  Main,
  #[default]
  Disabled,
}

#[derive(Resource)]
struct MenuNextState<T>(T);

pub trait MenuExtensions {
  fn add_main_menu<T: States + Copy>(&mut self, show_on_state: T, next_state: T) -> &mut Self;
}

impl MenuExtensions for App {
  fn add_main_menu<T: States + Copy>(&mut self, show_on_state: T, next_state: T) -> &mut Self {
    self
      .add_state::<MenuState>()
      .insert_resource(MenuNextState(next_state))
      .add_systems(OnEnter(show_on_state), menu_setup)
      .add_systems(OnEnter(MenuState::Main), main_menu_setup)
      .add_systems(Update, rotate_cam.run_if(in_state(show_on_state)))
      .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
      .add_systems(
        Update,
        (menu_action::<T>, button_system).run_if(in_state(show_on_state)),
      )
  }
}

fn menu_action<T: States>(
  interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
  mut app_exit_events: EventWriter<AppExit>,
  mut menu_state: ResMut<NextState<MenuState>>,
  mut game_state: ResMut<NextState<T>>,
  next_state: Res<MenuNextState<T>>,
) {
  for (interaction, menu_button_action) in &interaction_query {
    if *interaction == Interaction::Pressed {
      match menu_button_action {
        MenuButtonAction::Quit => app_exit_events.send(AppExit),
        MenuButtonAction::Play => {
          game_state.set(next_state.0.clone());
          menu_state.set(MenuState::Disabled);
        }
      }
    }
  }
}

#[derive(Component)]
struct OnMainMenuScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
enum MenuButtonAction {
  Play,
  Quit,
}

fn button_system(
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
    (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut color, selected) in &mut interaction_query {
    *color = match (*interaction, selected) {
      (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
      (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
      (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
      (Interaction::None, None) => NORMAL_BUTTON.into(),
    }
  }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
  menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let font = asset_server.load("fonts/FiraSans-Bold.ttf");
  commands.spawn((
    Camera3dBundle {
      transform: Transform::from_xyz(-2.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
      camera: Camera {
        hdr: true,
        order: 0,
        ..default()
      },
      ..default()
    },
    OnMainMenuScreen,
  ));

  commands.spawn((
    SceneBundle {
      scene: asset_server.load("ship.gltf#Scene0"),
      ..default()
    },
    OnMainMenuScreen,
  ));

  // Common style for all buttons on the screen
  let button_style = Style {
    width: Val::Px(250.0),
    height: Val::Px(65.0),
    margin: UiRect::all(Val::Px(20.0)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..default()
  };
  let button_icon_style = Style {
    width: Val::Px(30.0),
    // This takes the icons out of the flexbox flow, to be positioned exactly
    position_type: PositionType::Absolute,
    left: Val::Px(10.0),
    // The icon will be close to the left border of the button
    ..default()
  };
  let button_text_style = TextStyle {
    font: font.clone(),
    font_size: 40.0,
    color: TEXT_COLOR,
  };

  commands
    .spawn((
      NodeBundle {
        style: Style {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          flex_direction: FlexDirection::Column,
          align_items: AlignItems::Stretch,
          justify_content: JustifyContent::SpaceBetween,
          ..default()
        },
        ..default()
      },
      OnMainMenuScreen,
    ))
    .with_children(|parent| {
      parent
        .spawn(NodeBundle {
          style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            ..default()
          },
          ..default()
        })
        .with_children(|parent| {
          // Display the game name
          parent.spawn(
            TextBundle::from_section(
              "Name TBD",
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
      parent
        .spawn(NodeBundle {
          style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::End,
            ..default()
          },
          ..default()
        })
        .with_children(|parent| {
          parent
            .spawn((
              ButtonBundle {
                style: button_style.clone(),
                background_color: NORMAL_BUTTON.into(),
                ..default()
              },
              MenuButtonAction::Play,
            ))
            .with_children(|parent| {
              let icon = asset_server.load("ui/right.png");
              parent.spawn(ImageBundle {
                style: button_icon_style.clone(),
                image: UiImage::new(icon),
                ..default()
              });
              parent.spawn(TextBundle::from_section("Play", button_text_style.clone()));
            });

          parent
            .spawn((
              ButtonBundle {
                style: button_style,
                background_color: NORMAL_BUTTON.into(),
                ..default()
              },
              MenuButtonAction::Quit,
            ))
            .with_children(|parent| {
              let icon = asset_server.load("ui/exitRight.png");
              parent.spawn(ImageBundle {
                style: button_icon_style,
                image: UiImage::new(icon),
                ..default()
              });
              parent.spawn(TextBundle::from_section("Quit", button_text_style));
            });
        });
    });
}

fn rotate_cam(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
  for mut transform in &mut query {
    transform.rotate_around(
      Vec3::ZERO,
      Quat::from_axis_angle(Vec3::Y, 0.55 * time.delta_seconds()),
    );
  }
}
