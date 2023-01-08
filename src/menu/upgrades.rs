use bevy::prelude::*;

use crate::{despawn_screen, GameState, GameData};

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(GameState::Upgrades).with_system(setup))
      .add_system_set(
        SystemSet::on_update(GameState::Upgrades)
          .with_system(menu_action)
          .with_system(button_system),
      )
      .add_system_set(
        SystemSet::on_exit(GameState::Upgrades).with_system(despawn_screen::<OnMenuScreen>),
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
  IncreaseDamage,
  IncreaseFireRate,
  LevelSelect,
}

#[derive(Component)]
struct CurrentMoney;


fn menu_action(
  interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
  mut game_state: ResMut<State<GameState>>,
  mut data: ResMut<GameData>,
  mut money_display: Query<&mut Text, With<CurrentMoney>>,
) {
  for (interaction, menu_button_action) in &interaction_query {
    if *interaction == Interaction::Clicked {
      match menu_button_action {
        MenuButtonAction::IncreaseDamage => {
          if data.money >= 50 {
            data.gun_damage += 20.0;
            data.money -= 50;
            for mut display in money_display.iter_mut() {
              display.sections[0].value = format!("${:?}", data.money);
            }
          }
        },
        MenuButtonAction::IncreaseFireRate => {
          if data.money >= 50 {
            data.gun_cooldown = data.gun_cooldown * 0.9;
            data.money -= 50;
            for mut display in money_display.iter_mut() {
              display.sections[0].value = format!("${:?}", data.money);
            }
          }
        },
        MenuButtonAction::LevelSelect => {
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
  state: Res<GameData>,
) {
  let font = asset_server.load("font.ttf");
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
    )).with_children(|parent| {
      parent
        .spawn((
          ButtonBundle {
            style: button_style.clone(),
            background_color: NORMAL_BUTTON.into(),
            ..default()
          },
          MenuButtonAction::LevelSelect,
        ))
        .with_children(|parent| {
          parent.spawn(TextBundle::from_section(
            "Level selection",
            button_text_style.clone(),
          ));
        });

      parent.spawn((
        CurrentMoney,
        TextBundle::from_section(
          format!("${:?}", state.money),
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
        })
      ));

      parent.spawn((
        NodeBundle {
          style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
          },
          ..default()
        },
        OnMenuScreen,
      ))
      .with_children(|parent| {
        parent
          .spawn((
            ButtonBundle {
              style: button_style.clone(),
              background_color: NORMAL_BUTTON.into(),
              ..default()
            },
            MenuButtonAction::IncreaseFireRate,
          ))
          .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
              "Fire rate $50",
              button_text_style.clone(),
            ));
          });
  
        parent
          .spawn((
            ButtonBundle {
              style: button_style.clone(),
              background_color: NORMAL_BUTTON.into(),
              ..default()
            },
            MenuButtonAction::IncreaseDamage,
          ))
          .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
              "Damage $50",
              button_text_style.clone(),
            ));
          });
      });

    });
}
