use crate::components::*;
use crate::weapons::gun::*;
use crate::weapons::laser::*;
use crate::weapons::lightning::*;
use crate::{Action,GameData};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum Player {
  One,
  Two,
}

pub fn create_player(
  commands: &mut Commands,
  player: Player,
  texture_atlas: Handle<TextureAtlas>,
  input_manager: InputManagerBundle<Action>,
  asset_server: &Res<AssetServer>,
  data: &GameData,
) {
  commands
    .spawn((
      OnGameScreen,
      player,
      Gun {
        cooldown: Timer::from_seconds(data.gun_cooldown, TimerMode::Repeating),
        damage: data.gun_damage,
      },
      ActiveEvents::COLLISION_EVENTS,
      CollisionGroups::new(Group::GROUP_1, Group::GROUP_3.union(Group::GROUP_6)),
      SpriteSheetBundle {
        texture_atlas,
        ..default()
      },
      AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
      Velocity::default(),
      RigidBody::Dynamic,
    ))
    .insert((Collider::ball(10.), Restitution::coefficient(0.7)))
    .insert(input_manager)
    .with_children(|parent| {
      match player {
        Player::One => {
          parent.spawn((
            OnGameScreen,
            LightningGun {
              cooldown: Timer::from_seconds(6.0, TimerMode::Once),
              damage: 100.0,
              size: 2.5,
            },
            SpriteBundle {
              texture: asset_server.load("lightning_icon.png"),
              visibility: Visibility::INVISIBLE,
              transform: Transform {
                translation: Vec3::new(-16.0, 16.0, 0.0),
                scale: Vec3::new(0.7, 0.7, 0.7),
                ..default()
              },
              ..default()
            },
          ));
        }
        Player::Two => {
          parent.spawn((
            OnGameScreen,
            LaserGun {
              cooldown: Timer::from_seconds(8.0, TimerMode::Once),
              damage: 500.0,
            },
            SpriteBundle {
              texture: asset_server.load("laser_icon.png"),
              visibility: Visibility::INVISIBLE,
              transform: Transform {
                translation: Vec3::new(-16.0, 16.0, 0.0),
                scale: Vec3::new(0.7, 0.7, 0.7),
                ..default()
              },
              ..default()
            },
          ));
        }
      };
    });
}

pub fn player_movement(
  mut player_query: Query<(&mut Velocity, &ActionState<Action>), With<Player>>,
) {
  for (mut velocity, action_state) in player_query.iter_mut() {
    if action_state.pressed(Action::Move) {
      let mx_vec = action_state.clamped_axis_pair(Action::Move).unwrap().xy();
      let distance = 100.0;
      velocity.linvel.x = mx_vec.x * distance;
      velocity.linvel.y = mx_vec.y * distance;
    } else {
      velocity.linvel.x = 0.0;
      velocity.linvel.y = 0.0;
    }
  }
}
