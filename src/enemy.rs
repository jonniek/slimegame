use crate::components::*;
use crate::player::Player;
use crate::GameData;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

pub enum EnemySpawnerType {
  Normal,
  Elite,
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

#[derive(Resource, Default)]
pub struct EnemyAssets {
  pub atlas_handle: Handle<TextureAtlas>,
  pub elite_atlas_handle: Handle<TextureAtlas>,
}

#[derive(Component, Debug)]
pub struct Enemy;

pub fn generic_spawner(
  mut commands: Commands,
  mut spawners: Query<(&Transform, &mut EnemySpawner)>,
  enemy_assets: Res<EnemyAssets>,
  time: Res<Time>,
) {
  for (transform, mut spawner) in spawners.iter_mut() {
    spawner.initial_delay.tick(time.delta());

    if spawner.timer.tick(time.delta()).just_finished()
      && spawner.initial_delay.finished()
      && spawner.spawn_count < spawner.spawn_limit
    {
      let mut transform = Transform::from_translation(transform.translation);
      transform.translation.z += 0.1;

      spawner.spawn_count += 1;
      match spawner.enemy_type {
        EnemySpawnerType::Elite => spawn_enemy(
          &mut commands,
          transform,
          enemy_assets.elite_atlas_handle.clone(),
          500.0,
          EnemyMovement::Homing,
        ),
        EnemySpawnerType::Normal => {
          let mut rng = thread_rng();
          spawn_enemy(
            &mut commands,
            transform,
            enemy_assets.atlas_handle.clone(),
            100.0,
            EnemyMovement::Random(rng.gen_range(0.0..std::f32::consts::PI * 2.0)),
          );
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
) {
  commands
    .spawn((
      OnGameScreen,
      movement,
      Enemy,
      Health::new(health),
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
      Collider::ball(10.),
      Restitution::coefficient(0.7),
    ));
}

pub fn enemy_movement(
  mut query: Query<(&mut Transform, &mut Velocity, &mut EnemyMovement), Without<Player>>,
  players: Query<&Transform, With<Player>>,
) {
  let mut rng = thread_rng();
  for (transform, mut velocity, mut movement) in query.iter_mut() {
    match *movement {
      EnemyMovement::Homing => {
        let mut closest_player: Option<Vec3> = None;

        for player in players.iter() {
          match closest_player {
            Some(p) => {
              if transform.translation.distance(player.translation)
                < transform.translation.distance(p)
              {
                closest_player = Some(player.translation);
              }
            }
            None => closest_player = Some(player.translation),
          }
        }

        let speed = 50.0;

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
