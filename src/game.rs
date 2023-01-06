use bevy::prelude::*;

use super::{despawn_screen, GameState};
use crate::systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
    app
      // When entering the state, spawn everything needed for this screen
      // While in this state, run the `countdown` system
      //.add_startup_system(systems::setup_graphics)
      .add_system_set(SystemSet::on_enter(GameState::Game).with_system(systems::setup_graphics))
      .add_event::<systems::DamageEvent>()
      .add_system_set(
        SystemSet::on_update(GameState::Game)
          .with_system(systems::clean_up_expired)
          .with_system(systems::animate_sprite)
          .with_system(systems::sprite_movement)
          .with_system(systems::follow_camera.after(systems::sprite_movement))
          .with_system(systems::enemy_movement)
          .with_system(systems::spawn_projectiles)
          .with_system(systems::spawn_lightning)
          .with_system(systems::spawn_link)
          .with_system(systems::update_link.after(systems::spawn_link))
          .with_system(systems::kill_enemy)
          .with_system(systems::handle_laser_collision)
          .with_system(systems::handle_damage_event)
          .with_system(systems::enemy_homing_movement)
          .with_system(systems::spawner)
          .with_system(systems::elite_spawner)
          .with_system(systems::handle_collision)
          .with_system(systems::end_condition),
      )
      // When exiting the state, despawn everything that was spawned for this screen
      .add_system_set(
        SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>),
      );
  }
}

#[derive(Component)]
pub struct OnGameScreen;
