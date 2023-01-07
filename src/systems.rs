use crate::components::*;
use crate::enemy::*;
use crate::player::Player;
use crate::weapons::gun::Projectile;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent::Started;
use bevy_rapier2d::prelude::*;

use crate::{DamageEvent, GameData};

pub fn handle_damage_event(
  mut commands: Commands,
  mut damage_events: EventReader<DamageEvent>,
  mut enemies: Query<(&mut Health, &mut TextureAtlasSprite), With<Enemy>>,
  time: Res<Time>,
  mut state: ResMut<GameData>,
) {
  for damage_event in damage_events.iter() {
    if let Ok((mut health, mut sprite)) = enemies.get_mut(damage_event.entity) {
      health.current_health -= damage_event.damage;

      if health.current_health <= 0.0 {
        commands.entity(damage_event.entity).despawn();
        state.score += 1;
      } else {
        sprite.color.set_r(200.0);
        sprite.color.set_g(200.0);
        sprite.color.set_b(200.0);
        health.dmg_timer.reset();
        health.dmg_timer.unpause();
      }
    }
  }

  for (mut health, mut sprite) in enemies.iter_mut() {
    if health.dmg_timer.tick(time.delta()).just_finished() {
      sprite.color.set_r(1.0);
      sprite.color.set_g(1.0);
      sprite.color.set_b(1.0);
    }
  }
}

pub fn handle_collision(
  mut commands: Commands,
  projectiles: Query<&Projectile>,
  mut enemies: Query<(&mut Health, &mut TextureAtlasSprite), With<Enemy>>,
  mut collision_events: EventReader<CollisionEvent>,
  mut damage_event: EventWriter<DamageEvent>,
  player: Query<Entity, With<Player>>,
) {
  for collision in collision_events.iter() {
    match collision {
      Started(col1, col2, _) => {
        for (entity1, entity2) in [(col1, col2), (col2, col1)] {
          if let Ok(_) = player.get(*entity1) {
            commands.entity(*entity1).despawn_recursive();
          }

          if let Ok(data) = projectiles.get(*entity1) {
            let damage = data.damage;
            if let Ok(_) = enemies.get_mut(*entity2) {
              commands.entity(*entity1).despawn();
              damage_event.send(DamageEvent {
                entity: entity2.clone(),
                damage,
              });
            }
          }
        }
      }
      _ => (),
    }
  }
}

pub fn animate_sprite(
  time: Res<Time>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut query: Query<(
    &mut AnimationTimer,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>,
  )>,
) {
  for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
    timer.tick(time.delta());
    if timer.just_finished() {
      let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
      sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
    }
  }
}

pub fn clean_up_expired(
  time: Res<Time>,
  mut commands: Commands,
  mut query: Query<(Entity, &mut ExpirationTimer)>,
) {
  for (entity, mut timer) in query.iter_mut() {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
      commands.entity(entity).despawn();
    }
  }
}
