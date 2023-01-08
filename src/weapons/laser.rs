use crate::components::*;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::Action;
use crate::DamageEvent;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::FillMode;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Debug)]
pub struct LaserGunConfig {
  pub cooldown: f32,
  pub damage: f32,
}

#[derive(Component, Debug)]
pub struct LaserGun {
  pub cooldown: Timer,
  pub damage: f32,
}

impl LaserGun {
  pub fn from_config(config: &LaserGunConfig) -> LaserGun {
    LaserGun {
      cooldown: Timer::from_seconds(config.cooldown, TimerMode::Once),
      damage: config.damage,
    }
  }
}

#[derive(Component, Debug)]
pub struct Laser {
  pub damage: f32,
}

pub fn handle_laser_collision(
  rapier_context: Res<RapierContext>,
  laser_entities: Query<(Entity, &Laser)>,
  mut enemies: Query<(&mut Health, &mut TextureAtlasSprite), With<Enemy>>,
  mut damage_event: EventWriter<DamageEvent>,
  time: Res<Time>,
) {
  for (entity, laser) in laser_entities.iter() {
    for (col1, col2, intersecting) in rapier_context.intersections_with(entity) {
      if intersecting {
        for (entity1, _) in [(col1, col2), (col2, col1)] {
          if let Ok(_) = enemies.get_mut(entity1) {
            damage_event.send(DamageEvent {
              entity: entity1,
              damage: laser.damage * time.delta_seconds(),
            })
          }
        }
      }
    }
  }
}

pub fn spawn_laser(
  time: Res<Time>,
  mut commands: Commands,
  mut laser_gun_query: Query<(&Parent, &mut LaserGun, &mut Visibility)>,
  player_query: Query<&ActionState<Action>, With<Player>>,
) {
  for (parent, mut laser_gun, mut visibility) in laser_gun_query.iter_mut() {
    if let Ok(action_state) = player_query.get(parent.get()) {
      laser_gun.cooldown.tick(time.delta());

      if laser_gun.cooldown.just_finished() {
        *visibility = Visibility::VISIBLE;
      }

      let shape = shapes::Line {
        0: Vec2::new(0.0, 0.0),
        1: Vec2::new(0.0, 0.0),
      };

      if action_state.just_pressed(Action::Attack) && laser_gun.cooldown.finished() {
        *visibility = Visibility::INVISIBLE;

        laser_gun.cooldown.reset();
        commands.spawn((
          OnGameScreen,
          Laser {
            damage: laser_gun.damage,
          },
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

pub fn update_laser(
  mut commands: Commands,
  mut laser_query: Query<(Entity, &mut Path, &mut Collider), With<Laser>>,
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
    for (_, mut path, mut collider) in laser_query.iter_mut() {
      *path = ShapePath::build_as(&shape);
      let coll = Collider::polyline(vec![v1.clone(), v2.clone()], None);
      *collider = coll;
    }
  } else {
    for (entity, _, _) in laser_query.iter() {
      commands.entity(entity).despawn();
    }
  }
}
