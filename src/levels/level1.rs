use crate::camera;
use crate::components::*;
use crate::enemy::*;
use crate::map;
use crate::player::*;
use crate::systems;
use crate::weapons;
use crate::{
  despawn_screen, Action, DamageEvent, DespawnEvent, GameData, GameState, LevelEndTimer,
  TextureAtlasHandles,
};

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct Level1Plugin;

impl Plugin for Level1Plugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(GameState::Level1).with_system(init))
      .add_event::<DamageEvent>()
      .add_event::<DespawnEvent>()
      .add_system_set(
        SystemSet::on_update(GameState::Level1)
          .with_system(systems::clean_up_expired)
          .with_system(systems::animate_sprite)
          .with_system(player_movement)
          .with_system(camera::follow_camera.after(player_movement))
          .with_system(weapons::gun::spawn_projectiles)
          .with_system(weapons::lightning::spawn_lightning)
          .with_system(weapons::laser::spawn_laser)
          .with_system(weapons::laser::update_laser.after(weapons::laser::spawn_laser))
          .with_system(weapons::laser::handle_laser_collision)
          .with_system(systems::handle_damage_event)
          .with_system(enemy_movement)
          .with_system(generic_spawner)
          .with_system(handle_explosion)
          .with_system(systems::handle_collision)
          .with_system(systems::deal_red_zone_dmg)
          .with_system(systems::handle_despawn_entity)
          .with_system(end_condition),
      )
      // When exiting the state, despawn everything that was spawned for this screen
      .add_system_set(
        SystemSet::on_exit(GameState::Level1)
          .with_system(despawn_screen::<OnGameScreen>)
          .with_system(systems::save_game),
      );
  }
}

fn init(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  textures: Res<TextureAtlasHandles>,
  state: ResMut<GameData>,
  mut level_end_timer: ResMut<LevelEndTimer>,
) {
  level_end_timer.timer.reset();

  map::create_map_boundary(&mut commands);

  commands.spawn((
    OnGameScreen,
    SpriteBundle {
      texture: asset_server.load("cave.png"),
      transform: Transform {
        translation: Vec3::new(-150., -60., 0.0),
        ..default()
      },
      ..default()
    },
    EnemySpawner {
      timer: Timer::from_seconds(0.7, TimerMode::Repeating),
      initial_delay: Timer::from_seconds(2.0, TimerMode::Once),
      spawn_count: 0,
      spawn_limit: 30,
      enemy_type: EnemySpawnerType::Normal,
    },
  ));

  commands.spawn((
    OnGameScreen,
    SpriteBundle {
      texture: asset_server.load("cave.png"),
      transform: Transform {
        translation: Vec3::new(400., 400., 0.0),
        ..default()
      },
      ..default()
    },
    EnemySpawner {
      timer: Timer::from_seconds(0.01, TimerMode::Repeating),
      initial_delay: Timer::from_seconds(25.0, TimerMode::Once),
      spawn_count: 0,
      spawn_limit: 16,
      enemy_type: EnemySpawnerType::Elite,
    },
  ));

  create_player(
    &mut commands,
    Player::One,
    textures.player_atlas_handle.clone(),
    InputManagerBundle::<Action> {
      action_state: ActionState::default(),
      input_map: InputMap::default()
        .insert(VirtualDPad::arrow_keys(), Action::Move)
        .insert(KeyCode::Space, Action::Attack)
        .build(),
    },
    &asset_server,
    &state,
  );

  create_player(
    &mut commands,
    Player::Two,
    textures.player_atlas_handle.clone(),
    InputManagerBundle::<Action> {
      action_state: ActionState::default(),
      input_map: InputMap::default()
        .insert(VirtualDPad::wasd(), Action::Move)
        .insert(KeyCode::Q, Action::Attack)
        .build(),
    },
    &asset_server,
    &state,
  );
}

fn end_condition(
  players: Query<&Player>,
  enemies: Query<&Enemy>,
  spawners: Query<&EnemySpawner>,
  mut game_state: ResMut<State<GameState>>,
  mut data: ResMut<GameData>,
  mut level_end_timer: ResMut<LevelEndTimer>,
  time: Res<Time>,
) {
  // game won
  if enemies.is_empty()
    && spawners
      .iter()
      .all(|spawner| spawner.spawn_count >= spawner.spawn_limit)
  {
    if level_end_timer.timer.tick(time.delta()).finished() {
      if data.level < 2 {
        data.level = 2;
      }
      data.money += 50;
      game_state.set(GameState::LevelSelect).unwrap()
    }
  }

  // game lost
  if players.is_empty() {
    game_state.set(GameState::LevelSelect).unwrap()
  }
}
