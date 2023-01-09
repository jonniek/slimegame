use bevy::prelude::*;

use crate::{despawn_screen, GameState, GameData};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup))
      .add_system_set(
        SystemSet::on_update(GameState::MainMenu)
          .with_system(menu_action)
          .with_system(button_system),
      )
      .add_system_set(
        SystemSet::on_exit(GameState::MainMenu).with_system(despawn_screen::<OnMenuScreen>),
      );
  }
}

#[derive(Component)]
struct OnMenuScreen;

const HOVERED_BUTTON: Color = Color::rgb(0.15, 0.82, 0.2);
const NORMAL_BUTTON: Color = Color::rgb(0.20, 0.62, 0.27);

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Component)]
enum MenuButtonAction {
  NewGame,
  Continue,
}

fn menu_action(
  interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
  mut game_state: ResMut<State<GameState>>,
  mut game_data: ResMut<GameData>,
) {
  for (interaction, menu_button_action) in &interaction_query {
    if *interaction == Interaction::Clicked {
      match menu_button_action {
        MenuButtonAction::NewGame => {
          // start a fresh save
          *game_data = GameData::default();

          game_state.set(GameState::LevelSelect).unwrap();
        },
        MenuButtonAction::Continue => {
          game_state.set(GameState::LevelSelect).unwrap();
        },
      }
    }
  }
}

fn button_system(
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut color) in &mut interaction_query {
    *color = match *interaction {
      Interaction::Hovered | Interaction::Clicked => HOVERED_BUTTON.into(),
      Interaction::None => NORMAL_BUTTON.into(),
    }
  }
}

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  game_data: Res<GameData>,
) {
  let font = asset_server.load("font.ttf");
  let icon = asset_server.load("player.png");
  let button_text_style = TextStyle {
    font: font.clone(),
    font_size: 40.0,
    color: TEXT_COLOR,
  };
  let button_style = Style {
    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
    margin: UiRect::all(Val::Px(20.0)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..default()
  };

  commands
    .spawn((
      NodeBundle {
        style: Style {
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          flex_direction: FlexDirection::Column,
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
          ..default()
        },
        ..default()
      },
      OnMenuScreen,
    ))
    .with_children(|parent| {
      parent.spawn(ImageBundle {
        style: Style {
          size: Size::new(Val::Auto, Val::Px(200.0)),
          ..default()
        },
        image: UiImage::from(icon),
        ..default()
      });

      if !game_data.new_game {
        parent
          .spawn((
            ButtonBundle {
              style: button_style.clone(),
              background_color: NORMAL_BUTTON.into(),
              ..default()
            },
            MenuButtonAction::Continue,
          ))
          .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Continue game", button_text_style.clone()));
          });
      }

      parent
        .spawn((
          ButtonBundle {
            style: button_style.clone(),
            background_color: NORMAL_BUTTON.into(),
            ..default()
          },
          MenuButtonAction::NewGame,
        ))
        .with_children(|parent| {
          parent.spawn(TextBundle::from_section("New game", button_text_style.clone()));
        });
    });
}
