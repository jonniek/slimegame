use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

mod components;
mod game;
mod menu;
mod systems;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
  Menu,
  Game,
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
  for entity in &to_despawn {
    commands.entity(entity).despawn_recursive();
  }
}

#[derive(Resource, Default, Debug)]
pub struct GameData {
  score: usize,
  camera_pos: Vec2,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
  Move,
  Attack,
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
    .add_startup_system(systems::setup_camera)
    .add_system(bevy::window::close_on_esc)
    .add_state(GameState::Menu)
    .add_plugin(ShapePlugin)
    .add_plugin(InputManagerPlugin::<Action>::default())
    .add_plugin(menu::MenuPlugin)
    .add_plugin(game::GamePlugin)
    .run();
}