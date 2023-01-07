use crate::player::Player;
use crate::GameData;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

pub fn follow_camera(
  mut camera_query: Query<
    (
      &mut bevy::render::camera::OrthographicProjection,
      &mut Transform,
    ),
    Without<Player>,
  >,
  player_query: Query<&Transform, With<Player>>,
  mut state: ResMut<GameData>,
  time: Res<Time>,
) {
  let sum_position: Vec3 = player_query
    .iter()
    .map(|transform| transform.translation)
    .sum::<Vec3>();

  let average_position = sum_position * (1.0 / player_query.iter().count() as f32);
  let average_position_v2 = Vec2::new(average_position.x, average_position.y);

  if average_position_v2.distance(state.camera_pos) > 200.0 {
    state.camera_pos = average_position_v2;
  }

  for (_, mut pos) in camera_query.iter_mut() {
    let camera_v2 = Vec2::new(pos.translation.x, pos.translation.y);
    let distance = state.camera_pos.distance(camera_v2);

    if distance == 0.0 {
      continue;
    }

    let direction = (state.camera_pos - camera_v2).normalize_or_zero();

    pos.translation.x += direction.x * distance * time.delta_seconds();
    pos.translation.y += direction.y * distance * time.delta_seconds();
  }
}
