use crate::components::*;
use crate::player::Player;
use crate::GameData;
use crate::TextureAtlasHandles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

pub enum EnemySpawnerType {
  Normal,
  Elite,
  Boss,
}

#[derive(Component)]
pub struct EnemySpawner {
  pub timer: Timer,
  pub initial_delay: Timer,
  pub spawn_count: usize,
  pub spawn_limit: usize,
  pub enemy_type: EnemySpawnerType,
}

#[derive(Component)]
pub enum EnemyMovement {
  Homing,
  Random(f32),
}

#[derive(Component)]
pub struct EnemyEliteSpawner {
  pub timer: Timer,
}

#[derive(Component, Debug)]
pub struct Enemy {
  pub reward: i32,
}

#[derive(Component, Debug)]
pub struct Charge {
  pub cooldown: Timer,
  pub duration: Timer,
  pub direction: Vec2,
}

#[derive(Component, Debug)]
pub struct Explode;

pub fn generic_spawner(
  mut commands: Commands,
  mut spawners: Query<(&Transform, &mut EnemySpawner)>,
  textures: Res<TextureAtlasHandles>,
  time: Res<Time>,
) {
  for (transform, mut spawner) in spawners.iter_mut() {
    spawner.initial_delay.tick(time.delta());

    if !spawner.initial_delay.finished() {
      continue;
    }

    if spawner.timer.tick(time.delta()).just_finished() && spawner.spawn_count < spawner.spawn_limit
    {
      let mut transform = Transform::from_translation(transform.translation);
      transform.translation.z += 0.1;

      spawner.spawn_count += 1;
      match spawner.enemy_type {
        EnemySpawnerType::Elite => spawn_enemy(
          &mut commands,
          transform.with_scale(Vec3::new(1.25, 1.25, 1.25)),
          textures.elite_atlas_handle.clone(),
          500.0,
          EnemyMovement::Homing,
          5,
        ),
        EnemySpawnerType::Normal => {
          let mut rng = thread_rng();
          spawn_enemy(
            &mut commands,
            transform,
            textures.atlas_handle.clone(),
            100.0,
            EnemyMovement::Random(rng.gen_range(0.0..std::f32::consts::PI * 2.0)),
            1,
          );
        }
        EnemySpawnerType::Boss => {
          commands
            .spawn((
              OnGameScreen,
              EnemyMovement::Homing,
              Enemy { reward: 200 },
              Health::new(2000.0),
              Charge {
                cooldown: Timer::from_seconds(8.0, TimerMode::Repeating),
                duration: {
                  let mut t = Timer::from_seconds(2.0, TimerMode::Once);
                  t.pause();
                  t
                },
                direction: Vec2::default(),
              },
              Explode,
              ActiveEvents::COLLISION_EVENTS,
              CollisionGroups::new(Group::GROUP_3, Group::ALL),
              SpriteSheetBundle {
                texture_atlas: textures.boss_atlas_handle.clone(),
                transform: transform.with_scale(Vec3::new(3.0, 3.0, 3.0)),
                ..default()
              },
              AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            ))
            .insert((
              RigidBody::Dynamic,
              Velocity {
                linvel: Vec2::new(20., 20.),
                angvel: 0.0,
              },
              Damping {
                linear_damping: 0.0,
                angular_damping: 100000.0,
              },
              Collider::ball(12.),
              Restitution::coefficient(0.7),
            ));
        }
      };
    }
  }
}

pub fn spawn_enemy(
  commands: &mut Commands,
  transform: Transform,
  texture_atlas: Handle<TextureAtlas>,
  health: f32,
  movement: EnemyMovement,
  reward: i32,
) {
  commands
    .spawn((
      OnGameScreen,
      movement,
      Enemy { reward },
      Health::new(health),
      ActiveEvents::COLLISION_EVENTS,
      CollisionGroups::new(Group::GROUP_3, Group::ALL),
      SpriteSheetBundle {
        texture_atlas,
        transform,
        ..default()
      },
      AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
    ))
    .insert((
      RigidBody::Dynamic,
      Velocity {
        linvel: Vec2::new(20., 20.),
        angvel: 0.0,
      },
      Damping {
        linear_damping: 0.0,
        angular_damping: 100000.0,
      },
      Collider::ball(12.),
      Restitution::coefficient(0.7),
    ));
}

fn closest_player(
  transform: &Transform,
  players: &Query<&Transform, With<Player>>,
) -> Option<Vec3> {
  let mut closest_player: Option<Vec3> = None;

  for player in players.iter() {
    match closest_player {
      Some(p) => {
        if transform.translation.distance(player.translation) < transform.translation.distance(p) {
          closest_player = Some(player.translation);
        }
      }
      None => closest_player = Some(player.translation),
    }
  }

  closest_player
}

pub fn enemy_movement(
  mut query: Query<(&mut Transform, &mut Velocity, &mut EnemyMovement), Without<Player>>,
  players: Query<&Transform, With<Player>>,
) {
  let mut rng = thread_rng();
  for (transform, mut velocity, mut movement) in query.iter_mut() {
    match *movement {
      EnemyMovement::Homing => {
        let closest_player = closest_player(&transform, &players);

        let speed = 60.0;

        match closest_player {
          Some(player) => {
            let direction = (player - transform.translation).normalize_or_zero();
            velocity.linvel = Vec2::new(direction.x * speed, direction.y * speed);
          }
          None => (),
        }
      }
      EnemyMovement::Random(direction) => {
        let lower = direction - 0.3;
        let upper = direction + 0.3;

        let new_direction = rng.gen_range(lower..upper);

        *movement = EnemyMovement::Random(new_direction);

        let x = rng.gen_range(0.0..75.0) * new_direction.cos();
        let y = rng.gen_range(0.0..75.0) * new_direction.sin();

        velocity.linvel.x = x;
        velocity.linvel.y = y;
      }
    }
  }
}

pub fn handle_charge(
  mut query: Query<(&mut Charge, &mut Velocity, &Transform), With<Enemy>>,
  players: Query<&Transform, With<Player>>,
  time: Res<Time>,
) {
  for (mut charge, mut velocity, transform) in query.iter_mut() {
    if charge.cooldown.tick(time.delta()).just_finished() {
      charge.duration.reset();
      charge.duration.unpause();
      let speed = 300.0;
      match closest_player(&transform, &players) {
        Some(player) => {
          let direction = (player - transform.translation).normalize_or_zero();
          charge.direction = Vec2::new(direction.x * speed, direction.y * speed);
        }
        None => (),
      }
    }
    if !charge.duration.paused() && !charge.duration.tick(time.delta()).finished() {
      velocity.linvel = charge.direction;
    }
  }
}

pub fn handle_explosion(
  mut commands: Commands,
  enemies: Query<(Entity, &Enemy, &Health, &Transform), With<Explode>>,
  mut state: ResMut<GameData>,
  textures: Res<TextureAtlasHandles>,
) {
  for (entity, enemy, health, transform) in enemies.iter() {
    if health.current_health <= 0.0 {
      state.money += enemy.reward;

      let mut rng = thread_rng();

      for _ in 0..30 {
        commands
          .spawn((
            OnGameScreen,
            EnemyMovement::Random(rng.gen_range(0.0..2.0 * std::f32::consts::PI)),
            Enemy { reward: 1 },
            Health::new(100.0),
            Charge {
              cooldown: Timer::from_seconds(rng.gen_range(4.0..10.0), TimerMode::Repeating),
              duration: {
                let mut t = Timer::from_seconds(2.0, TimerMode::Once);
                t.pause();
                t
              },
              direction: Vec2::default(),
            },
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(Group::GROUP_3, Group::ALL),
            SpriteSheetBundle {
              texture_atlas: textures.boss_atlas_handle.clone(),
              transform: transform.with_scale(Vec3::new(0.75, 0.75, 0.75)),
              ..default()
            },
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
          ))
          .insert((
            RigidBody::Dynamic,
            Velocity {
              linvel: Vec2::default(),
              angvel: 0.0,
            },
            Damping {
              linear_damping: 0.0,
              angular_damping: 100000.0,
            },
            Collider::ball(12.),
            Restitution::coefficient(0.7),
          ));
      }

      commands.entity(entity).despawn();
    }
  }
}
