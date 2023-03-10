use crate::components::*;
use crate::enemy::*;
use crate::player::Player;
use crate::weapons::gun::Projectile;
use bevy::prelude::*;
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::CollisionEvent::Started;
use bevy_rapier2d::prelude::*;

use crate::{DamageEvent, DespawnEvent, GameData, TextureAtlasHandles};

pub fn save_game(mut data: ResMut<GameData>, mut pkv: ResMut<PkvStore>) {
  data.new_game = false;
  match pkv.set("game_save", data.as_ref()) {
    Ok(_) => println!("Game quick saved"),
    Err(e) => eprintln!("Game quick save failed: {}", e),
  }
}

pub fn load_game(mut data: ResMut<GameData>, pkv: ResMut<PkvStore>) {
  match pkv.get::<GameData>("game_save") {
    Ok(save) => {
      println!("game loaded {:?}", save);
      *data = save
    }
    Err(e) => eprintln!("Game load failed: {}", e),
  }
}

pub fn initialize_texture_atlas(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture_handle = asset_server.load("player_96x32.png");
  let texture_atlas =
    TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 1, None, None);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  let texture_handle_enemy = asset_server.load("enemy_96x32.png");
  let texture_atlas_enemy = TextureAtlas::from_grid(
    texture_handle_enemy,
    Vec2::new(32.0, 32.0),
    3,
    1,
    None,
    None,
  );
  let texture_atlas_handle_enemy = texture_atlases.add(texture_atlas_enemy);

  let texture_handle_enemy2 = asset_server.load("enemy_2_96x32.png");
  let texture_atlas_enemy2 = TextureAtlas::from_grid(
    texture_handle_enemy2,
    Vec2::new(32.0, 32.0),
    3,
    1,
    None,
    None,
  );
  let texture_atlas_handle_enemy2 = texture_atlases.add(texture_atlas_enemy2);

  let texture_handle_enemy3 = asset_server.load("enemy_3_96x32.png");
  let texture_atlas_enemy3 = TextureAtlas::from_grid(
    texture_handle_enemy3,
    Vec2::new(32.0, 32.0),
    3,
    1,
    None,
    None,
  );
  let texture_atlas_handle_enemy3 = texture_atlases.add(texture_atlas_enemy3);

  commands.insert_resource(TextureAtlasHandles {
    atlas_handle: texture_atlas_handle_enemy,
    elite_atlas_handle: texture_atlas_handle_enemy2,
    boss_atlas_handle: texture_atlas_handle_enemy3,
    player_atlas_handle: texture_atlas_handle,
  });
}

pub fn handle_despawn_entity(
  mut commands: Commands,
  mut despawn_events: EventReader<DespawnEvent>,
  enemy_query: Query<(&Enemy, Option<&Explode>)>,
  mut state: ResMut<GameData>,
) {
  for event in despawn_events.iter() {
    match enemy_query.get(event.entity) {
      Ok((enemy, explosive)) => {
        if explosive.is_none() {
          state.money += enemy.reward;
          match commands.get_entity(event.entity) {
            Some(cmd) => cmd.despawn_recursive(),
            None => (),
          }
        }
      }
      Err(_) => match commands.get_entity(event.entity) {
        Some(cmd) => cmd.despawn_recursive(),
        None => (),
      },
    }
  }
}

pub fn handle_damage_event(
  mut damage_events: EventReader<DamageEvent>,
  mut entities_with_health: Query<(&mut Health, &mut TextureAtlasSprite)>,
  time: Res<Time>,
  mut despawn_events: EventWriter<DespawnEvent>,
) {
  for damage_event in damage_events.iter() {
    if let Ok((mut health, mut sprite)) = entities_with_health.get_mut(damage_event.entity) {
      health.current_health -= damage_event.damage;

      if health.current_health <= 0.0 {
        despawn_events.send(DespawnEvent {
          entity: damage_event.entity,
        });
      } else {
        sprite.color.set_r(200.0);
        sprite.color.set_g(200.0);
        sprite.color.set_b(200.0);
        health.dmg_timer.reset();
        health.dmg_timer.unpause();
      }
    }
  }

  for (mut health, mut sprite) in entities_with_health.iter_mut() {
    if health.dmg_timer.tick(time.delta()).just_finished() {
      sprite.color.set_r(1.0);
      sprite.color.set_g(1.0);
      sprite.color.set_b(1.0);
    }
  }
}

pub fn deal_red_zone_dmg(
  mut damage_event: EventWriter<DamageEvent>,
  killzone_query: Query<&CollidingEntities, With<Killzone>>,
  entities_with_health: Query<Entity, With<Health>>,
  time: Res<Time>,
) {
  for killzone_collision in killzone_query.iter() {
    for entity in killzone_collision.iter() {
      if let Ok(health_entity) = entities_with_health.get(entity) {
        damage_event.send(DamageEvent {
          entity: health_entity.clone(),
          damage: 20.0 * time.delta_seconds(),
        });
      }
    }
  }
}

pub fn handle_collision(
  projectiles: Query<&Projectile>,
  mut enemies: Query<(&mut Health, &mut TextureAtlasSprite), With<Enemy>>,
  mut collision_events: EventReader<CollisionEvent>,
  mut damage_event: EventWriter<DamageEvent>,
  mut despawn_event: EventWriter<DespawnEvent>,
  player: Query<Entity, With<Player>>,
) {
  for collision in collision_events.iter() {
    match collision {
      Started(col1, col2, _) => {
        for (entity1, entity2) in [(col1, col2), (col2, col1)] {
          if let Ok(player) = player.get(*entity1) {
            if enemies.get(*entity2).is_ok() {
              // player collision with enemy
              damage_event.send(DamageEvent {
                entity: player,
                damage: 10000.0,
              });
            }
          }

          if let Ok(data) = projectiles.get(*entity1) {
            let damage = data.damage;
            if let Ok(_) = enemies.get_mut(*entity2) {
              despawn_event.send(DespawnEvent { entity: *entity1 });
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
  mut query: Query<(Entity, &mut ExpirationTimer)>,
  mut despawn_events: EventWriter<DespawnEvent>,
) {
  for (entity, mut timer) in query.iter_mut() {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
      despawn_events.send(DespawnEvent { entity })
    }
  }
}
