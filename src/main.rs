use bevy::prelude::*;
use bevy_pkv::PkvStore;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod components;
mod enemy;
mod levels;
mod map;
mod menu;
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
  Level3,
}

#[derive(Resource, Default)]
pub struct TextureAtlasHandles {
  pub atlas_handle: Handle<TextureAtlas>,
  pub elite_atlas_handle: Handle<TextureAtlas>,
  pub boss_atlas_handle: Handle<TextureAtlas>,
  pub player_atlas_handle: Handle<TextureAtlas>,
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
  for entity in &to_despawn {
    commands.entity(entity).despawn_recursive();
  }
}

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct GameData {
  new_game: bool,
  level: usize,
  money: i32,
  camera_pos: Vec2,
  gun_cooldown: f32,
  gun_damage: f32,
  lightning_gun: weapons::lightning::LightningGunConfig,
  laser_gun: weapons::laser::LaserGunConfig,
}

impl Default for GameData {
  fn default() -> Self {
    GameData {
      new_game: true,
      level: 1,
      money: 200,
      camera_pos: Vec2::default(),
      gun_cooldown: 1.5,
      gun_damage: 20.0,
      lightning_gun: weapons::lightning::LightningGunConfig {
        cooldown: 10.0,
        damage: 100.0,
        size: 2.5,
      },
      laser_gun: weapons::laser::LaserGunConfig {
        cooldown: 10.0,
        damage: 500.0,
      },
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

pub struct DespawnEvent {
  entity: Entity,
}

#[derive(Resource)]
pub struct LevelEndTimer {
  timer: Timer,
}

impl Default for LevelEndTimer {
  fn default() -> Self {
    LevelEndTimer {
      timer: Timer::from_seconds(3.0, TimerMode::Once),
    }
  }
}

fn main() {
  App::new()
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          window: WindowDescriptor {
            title: "slimegame".into(),
            ..default()
          },
          ..default()
        })
        .set(ImagePlugin::default_nearest()),
    )
    .add_plugin(WorldInspectorPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .insert_resource(RapierConfiguration {
      gravity: Vec2::ZERO,
      ..default()
    })
    .insert_resource(ClearColor(Color::rgb(0.7, 0.6, 0.5)))
    // .add_plugin(RapierDebugRenderPlugin::default())
    .init_resource::<GameData>()
    .init_resource::<LevelEndTimer>()
    .insert_resource(PkvStore::new("Slime", "Game"))
    .add_startup_system(camera::setup_camera)
    .add_startup_system(systems::initialize_texture_atlas)
    .add_startup_system(systems::load_game)
    .add_system(bevy::window::close_on_esc)
    .add_state(GameState::MainMenu)
    .add_plugin(ShapePlugin)
    .add_plugin(InputManagerPlugin::<Action>::default())
    .add_plugin(menu::main_menu::MainMenuPlugin)
    .add_plugin(menu::level_select::LevelSelectPlugin)
    .add_plugin(menu::upgrades::UpgradesPlugin)
    .add_plugin(levels::level1::Level1Plugin)
    .add_plugin(levels::level2::Level2Plugin)
    .add_plugin(levels::level3::Level3Plugin)
    .run();
}
