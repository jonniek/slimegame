use crate::components::*;
use crate::enemy::*;
use crate::player::Player;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::FillMode;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent::Started;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use rand::prelude::*;

use super::{Action, GameData};

pub struct DamageEvent {
  entity: Entity,
  damage: f32,
}

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

pub fn handle_laser_collision(
  rapier_context: Res<RapierContext>,
  link_entities: Query<(Entity, &Link)>,
  mut enemies: Query<(&mut Health, &mut TextureAtlasSprite), With<Enemy>>,
  mut damage_event: EventWriter<DamageEvent>,
  time: Res<Time>,
) {
  for (entity, link) in link_entities.iter() {
    for (col1, col2, intersecting) in rapier_context.intersections_with(entity) {
      if intersecting {
        for (entity1, _) in [(col1, col2), (col2, col1)] {
          if let Ok(_) = enemies.get_mut(entity1) {
            damage_event.send(DamageEvent {
              entity: entity1,
              damage: link.damage * time.delta_seconds(),
            })
          }
        }
      }
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

pub fn kill_enemy(
  mut commands: Commands,
  enemies: Query<(Entity, &Health), With<Enemy>>,
  mut state: ResMut<GameData>,
) {
  for (entity, health) in enemies.iter() {
    if health.current_health <= 0.0 {
      commands.entity(entity).despawn();
      state.score += 1;
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

pub fn spawn_link(
  time: Res<Time>,
  mut commands: Commands,
  mut link_gun_query: Query<(&Parent, &mut LinkGun, &mut Visibility)>,
  player_query: Query<&ActionState<Action>, With<Player>>,
) {
  //  let parent_global_transform = q_parent.get(parent.get());
  for (parent, mut link_gun, mut visibility) in link_gun_query.iter_mut() {
    if let Ok(action_state) = player_query.get(parent.get()) {
      link_gun.cooldown.tick(time.delta());

      if link_gun.cooldown.just_finished() {
        *visibility = Visibility::VISIBLE;
      }

      let shape = shapes::Line {
        0: Vec2::new(0.0, 0.0),
        1: Vec2::new(0.0, 0.0),
      };

      if action_state.just_pressed(Action::Attack) && link_gun.cooldown.finished() {
        *visibility = Visibility::INVISIBLE;

        link_gun.cooldown.reset();
        commands.spawn((
          OnGameScreen,
          Link { damage: 500.0 },
          ExpirationTimer(Timer::from_seconds(1.75, TimerMode::Once)),
          ActiveEvents::COLLISION_EVENTS,
          Sensor,
          CollisionGroups::new(Group::GROUP_5, Group::GROUP_3),
          Collider::polyline(vec![Vec2::default(), Vec2::default()], None),
          GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
              fill_mode: FillMode::color(Color::CYAN),
              outline_mode: StrokeMode::new(Color::CYAN, 3.0),
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
          ),
        ));
      }
    }
  }
}

pub fn update_link(
  mut link_query: Query<(&mut Path, &mut Collider), With<Link>>,
  players_query: Query<&Transform, With<Player>>,
) {
  if players_query.iter().count() >= 2 {
    let mut player_iter = players_query.iter();
    let p1 = player_iter.next().unwrap();
    let p2 = player_iter.next().unwrap();
    let v1 = Vec2::new(p1.translation.x, p1.translation.y);
    let v2 = Vec2::new(p2.translation.x, p2.translation.y);
    let shape = shapes::Line {
      0: v1.clone(),
      1: v2.clone(),
    };
    for (mut path, mut collider) in link_query.iter_mut() {
      *path = ShapePath::build_as(&shape);
      let coll = Collider::polyline(vec![v1.clone(), v2.clone()], None);
      *collider = coll;
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
          let scale = 2.5;

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
                damage: 100.0,
              });
            }
          }
          lightning_gun.cooldown.reset();
        }
      }
    }
  }
}

pub fn spawn_projectiles(
  time: Res<Time>,
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut player_query: Query<(Entity, &Player, &Transform, &mut Gun)>,
) {
  let mut rng = thread_rng();
  let random_angle: f32 = rng.gen_range(0.0..std::f32::consts::PI * 2.0);

  let direction = Vec2::new(random_angle.cos(), random_angle.sin());

  for (_, _, player_transform, mut gun) in player_query.iter_mut() {
    gun.cooldown.tick(time.delta());

    let mut new_transform = Transform::from_translation(player_transform.translation);
    new_transform.translation += Vec3::new(direction.x * 20.0, direction.y * 20.0, 0.0);

    if gun.cooldown.just_finished() {
      commands.spawn((
        OnGameScreen,
        SpriteBundle {
          texture: asset_server.load("projectile.png"),
          transform: new_transform,
          ..default()
        },
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
        Projectile { damage: 50.0 },
        ExpirationTimer(Timer::from_seconds(10.0, TimerMode::Once)),
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Dynamic,
        Velocity {
          linvel: direction * 400.0,
          angvel: 0.0,
        },
        Collider::ball(3.),
      ));
    }
  }
}
