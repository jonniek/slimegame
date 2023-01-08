use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

mod camera;
mod components;
mod enemy;
mod menu;
mod levels;
mod player;
mod systems;
mod weapons;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
  MainMenu,
  LevelSelect,
  Upgrades,
  Level1,
  Level2,
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
  for entity in &to_despawn {
    commands.entity(entity).despawn_recursive();
  }
}

#[derive(Resource, Debug)]
pub struct GameData {
  money: i32,
  camera_pos: Vec2,
  gun_cooldown: f32,
  gun_damage: f32,
}

impl Default for GameData {
  fn default() -> Self {
      GameData {
        money: 100,
        camera_pos: Vec2::default(),
        gun_cooldown: 1.5,
        gun_damage: 20.0,
      }
  }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
  Move,
  Attack,
}

pub struct DamageEvent {
  entity: Entity,
  damage: f32,
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      window: WindowDescriptor {
        title: "slimegame".into(),
        ..default()
      },
      ..default()
    }))
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .insert_resource(RapierConfiguration {
      gravity: Vec2::ZERO,
      ..default()
    })
    .insert_resource(ClearColor(Color::rgb(0.7, 0.6, 0.5)))
    // .add_plugin(RapierDebugRenderPlugin::default())
    .init_resource::<GameData>()
    .add_startup_system(camera::setup_camera)
    .add_system(bevy::window::close_on_esc)
    .add_state(GameState::MainMenu)
    .add_plugin(ShapePlugin)
    .add_plugin(InputManagerPlugin::<Action>::default())
    .add_plugin(menu::main_menu::MainMenuPlugin)
    .add_plugin(menu::level_select::LevelSelectPlugin)
    .add_plugin(menu::upgrades::UpgradesPlugin)
    .add_plugin(levels::level1::Level1Plugin)
    .add_plugin(levels::level2::Level2Plugin)
    .run();
}
