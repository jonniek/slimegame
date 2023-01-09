use crate::components::*;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::Action;
use crate::DamageEvent;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LightningGunConfig {
  pub cooldown: f32,
  pub damage: f32,
  pub size: f32,
}

#[derive(Component, Debug)]
pub struct LightningGun {
  pub cooldown: Timer,
  pub damage: f32,
  pub size: f32,
}

impl LightningGun {
  pub fn from_config(config: &LightningGunConfig) -> LightningGun {
    LightningGun {
      cooldown: Timer::from_seconds(config.cooldown, TimerMode::Once),
      damage: config.damage,
      size: config.size,
    }
  }
}

pub fn spawn_lightning(
  time: Res<Time>,
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut lightning_gun_query: Query<(&Parent, &mut LightningGun, &mut Visibility)>,
  player_query: Query<(&ActionState<Action>, &Transform), With<Player>>,
  enemy_query: Query<(Entity, &Transform), With<Enemy>>,
  mut damage_event: EventWriter<DamageEvent>,
) {
  for (parent, mut lightning_gun, mut visibility) in lightning_gun_query.iter_mut() {
    lightning_gun.cooldown.tick(time.delta());

    if lightning_gun.cooldown.just_finished() {
      *visibility = Visibility::VISIBLE;
    }

    if let Ok((action_state, player_transform)) = player_query.get(parent.get()) {
      if action_state.just_pressed(Action::Attack) {
        if lightning_gun.cooldown.finished() {
          *visibility = Visibility::INVISIBLE;
          let scale = lightning_gun.size;

          commands.spawn((
            OnGameScreen,
            SpriteBundle {
              texture: asset_server.load("lightning.png"),
              transform: player_transform.with_scale(Vec3::new(scale, scale, scale)),
              ..default()
            },
            ExpirationTimer(Timer::from_seconds(0.1, TimerMode::Once)),
          ));

          for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance = player_transform
              .translation
              .distance(enemy_transform.translation);
            if distance < 75.0 * scale / 2.0 {
              damage_event.send(DamageEvent {
                entity: enemy_entity,
                damage: lightning_gun.damage,
              });
            }
          }
          lightning_gun.cooldown.reset();
        }
      }
    }
  }
}
