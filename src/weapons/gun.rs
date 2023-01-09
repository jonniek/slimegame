use crate::components::*;
use crate::enemy::Enemy;
use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

#[derive(Component, Debug)]
pub struct Projectile {
  pub damage: f32,
}

#[derive(Component, Debug)]
pub struct Gun {
  pub cooldown: Timer,
  pub damage: f32,
}

fn closest_enemy(transform: &Transform, enemies: &Query<&Transform, With<Enemy>>) -> Option<Vec3> {
  let mut closest: Option<Vec3> = None;

  for player in enemies.iter() {
    match closest {
      Some(p) => {
        if transform.translation.distance(player.translation) < transform.translation.distance(p) {
          closest = Some(player.translation);
        }
      }
      None => closest = Some(player.translation),
    }
  }

  closest
}

pub fn spawn_projectiles(
  time: Res<Time>,
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut player_query: Query<(Entity, &Player, &Transform, &mut Gun)>,
  enemy_query: Query<&Transform, With<Enemy>>,
) {
  let mut rng = thread_rng();
  let random_angle: f32 = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
  let random_direction = Vec3::new(random_angle.cos(), random_angle.sin(), 0.0);

  for (_, _, player_transform, mut gun) in player_query.iter_mut() {
    let closest_target = closest_enemy(player_transform, &enemy_query).unwrap_or(random_direction);
    let direction = (closest_target - player_transform.translation).normalize_or_zero();

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
        Projectile { damage: gun.damage },
        ExpirationTimer(Timer::from_seconds(5.0, TimerMode::Once)),
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Dynamic,
        Velocity {
          linvel: Vec2::new(direction.x, direction.y) * 400.0,
          angvel: 0.0,
        },
        Collider::ball(3.),
      ));
    }
  }
}
