use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy_prototype_lyon::prelude::FillMode;
use bevy_prototype_lyon::prelude::*;

pub fn create_map_boundary(commands: &mut Commands) {
  for n in -30..=30 {
    let n = n as f32 * 50.0;
    let shape1 = shapes::Line {
      0: Vec2::new(n, 5000.0),
      1: Vec2::new(n, -5000.0),
    };

    let shape2 = shapes::Line {
      0: Vec2::new(-5000.0, n),
      1: Vec2::new(5000.0, n),
    };

    commands.spawn((
      OnGameScreen,
      GeometryBuilder::build_as(
        &shape1,
        DrawMode::Outlined {
          fill_mode: FillMode::color(Color::BLACK),
          outline_mode: StrokeMode::new(Color::rgba(0.0, 0.0, 0.0, 0.2), 1.0),
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
      ),
    ));
    commands.spawn((
      OnGameScreen,
      GeometryBuilder::build_as(
        &shape2,
        DrawMode::Outlined {
          fill_mode: FillMode::color(Color::BLACK),
          outline_mode: StrokeMode::new(Color::rgba(0.0, 0.0, 0.0, 0.1), 1.0),
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
      ),
    ));
  }

  let map_size = 600.0;

  let map_size_50 = map_size * 50.0;
  let map_size_25 = map_size * 25.0;

  for (translate, size, cuboid) in [
    (
      (0.0, -map_size - map_size_25, 0.0),
      (map_size_50, map_size_50),
      (map_size_25, map_size_25),
    ),
    (
      (0.0, map_size + map_size_25, 0.0),
      (map_size_50, map_size_50),
      (map_size_25, map_size_25),
    ),
    (
      (-map_size - map_size_25, 0.0, 0.0),
      (map_size_50, map_size * 2.0),
      (map_size_25, map_size_25),
    ),
    (
      (map_size + map_size_25, 0.0, 0.0),
      (map_size_50, map_size * 2.0),
      (map_size_25, map_size_25),
    ),
  ] {
    let square = shapes::Rectangle {
      extents: Vec2::new(size.0, size.1),
      origin: RectangleOrigin::Center,
    };
    commands.spawn((
      OnGameScreen,
      Killzone,
      GeometryBuilder::build_as(
        &square,
        DrawMode::Outlined {
          fill_mode: FillMode::color(Color::rgba(1.0, 0.0, 0.0, 0.3)),
          outline_mode: StrokeMode::new(Color::rgba(1.0, 0.0, 0.0, 0.0), 0.0),
        },
        Transform::from_translation(Vec3::new(translate.0, translate.1, translate.2)),
      ),
      Collider::cuboid(cuboid.0, cuboid.1),
      Sensor,
      CollidingEntities::default(),
      RigidBody::Fixed,
      CollisionGroups::new(Group::GROUP_6, Group::ALL),
    ));
  }
}
