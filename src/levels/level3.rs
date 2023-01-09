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

pub struct Level3Plugin;

impl Plugin for Level3Plugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(GameState::Level3).with_system(init))
      .add_event::<DamageEvent>()
      .add_event::<DespawnEvent>()
      .add_system_set(
        SystemSet::on_update(GameState::Level3)
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
          .with_system(handle_charge.after(enemy_movement))
          .with_system(generic_spawner)
          .with_system(handle_explosion)
          .with_system(systems::handle_collision)
          .with_system(systems::handle_despawn_entity.at_end())
          .with_system(end_condition.at_end()),
      )
      // When exiting the state, despawn everything that was spawned for this screen
      .add_system_set(
        SystemSet::on_exit(GameState::Level3)
          .with_system(despawn_screen::<OnGameScreen>)
          .with_system(systems::save_game),
      );
  }
}

pub fn init(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  textures: Res<TextureAtlasHandles>,
  state: Res<GameData>,
  mut level_end_timer: ResMut<LevelEndTimer>,
) {
  level_end_timer.timer.reset();

  map::create_map_boundary(&mut commands);

  commands.spawn((
    OnGameScreen,
    SpriteBundle {
      texture: asset_server.load("cave.png"),
      transform: Transform {
        translation: Vec3::new(0., 0., 0.0),
        ..default()
      },
      ..default()
    },
    EnemySpawner {
      timer: Timer::from_seconds(1.0, TimerMode::Repeating),
      initial_delay: Timer::from_seconds(5.0, TimerMode::Once),
      spawn_count: 0,
      spawn_limit: 1,
      enemy_type: EnemySpawnerType::Boss,
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
  time: Res<Time>,
  mut level_end_timer: ResMut<LevelEndTimer>,
) {
  // game won
  if enemies.is_empty()
    && spawners
      .iter()
      .all(|spawner| spawner.spawn_count >= spawner.spawn_limit)
  {
    if level_end_timer.timer.tick(time.delta()).finished() {
      game_state.set(GameState::LevelSelect).unwrap()
    }
  }

  // game lost
  if players.is_empty() {
    game_state.set(GameState::LevelSelect).unwrap()
  }
}
