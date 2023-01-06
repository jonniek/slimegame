use bevy::prelude::*;

use super::{despawn_screen, GameData, GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(manu_setup))
      .add_system_set(
        SystemSet::on_update(GameState::Menu)
          .with_system(menu_action)
          .with_system(button_system),
      )
      .add_system_set(
        SystemSet::on_exit(GameState::Menu).with_system(despawn_screen::<OnMenuScreen>),
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
  Play,
}

fn menu_action(
  interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
  mut game_state: ResMut<State<GameState>>,
) {
  for (interaction, menu_button_action) in &interaction_query {
    if *interaction == Interaction::Clicked {
      match menu_button_action {
        MenuButtonAction::Play => {
          game_state.set(GameState::Game).unwrap();
        }
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

fn manu_setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<GameData>) {
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
      if state.score > 0 {
        parent.spawn(
          TextBundle::from_section(
            format!("Game over!\nScore: {:?}", state.score),
            TextStyle {
              font: font.clone(),
              font_size: 40.0,
              color: TEXT_COLOR,
            },
          )
          .with_text_alignment(TextAlignment::CENTER)
          .with_style(Style {
            margin: UiRect::all(Val::Px(50.0)),
            ..default()
          }),
        );
      }

      parent.spawn(ImageBundle {
        style: Style {
          size: Size::new(Val::Auto, Val::Px(200.0)),
          ..default()
        },
        image: UiImage::from(icon),
        ..default()
      });

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
          parent.spawn(TextBundle::from_section("Play", button_text_style.clone()));
        });
    });
}
