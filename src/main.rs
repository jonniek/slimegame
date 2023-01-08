use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod components;
mod enemy;
mod levels;
mod menu;
mod player;
mod systems;
mod weapons;
mod map;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
  MainMenu,
  LevelSelect,
  Upgrades,
  Level1,
  Level2,
}

#[derive(Resource, Default)]
pub struct TextureAtlasHandles {
  pub atlas_handle: Handle<TextureAtlas>,
  pub elite_atlas_handle: Handle<TextureAtlas>,
  pub player_atlas_handle: Handle<TextureAtlas>,
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
  lightning_gun: weapons::lightning::LightningGunConfig,
  laser_gun: weapons::laser::LaserGunConfig,
}

impl Default for GameData {
  fn default() -> Self {
    GameData {
      money: 1000,
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
    .add_startup_system(camera::setup_camera)
    .add_startup_system(systems::initialize_texture_atlas)
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
