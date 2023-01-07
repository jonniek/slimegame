use crate::components::*;
use crate::systems;
use crate::{Action, GameData, GameState, despawn_screen};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::FillMode;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;


pub struct Level2Plugin;

impl Plugin for Level2Plugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(GameState::Level2).with_system(init))
      .add_event::<systems::DamageEvent>()
      .add_system_set(
        SystemSet::on_update(GameState::Level2)
          .with_system(systems::clean_up_expired)
          .with_system(systems::animate_sprite)
          .with_system(systems::sprite_movement)
          .with_system(systems::follow_camera.after(systems::sprite_movement))
          .with_system(systems::enemy_movement)
          .with_system(systems::spawn_projectiles)
          .with_system(systems::spawn_lightning)
          .with_system(systems::spawn_link)
          .with_system(systems::update_link.after(systems::spawn_link))
          .with_system(systems::kill_enemy)
          .with_system(systems::handle_laser_collision)
          .with_system(systems::handle_damage_event)
          .with_system(systems::enemy_movement)
          .with_system(systems::generic_spawner)
          .with_system(systems::handle_collision)
          .with_system(end_condition),
      )
      // When exiting the state, despawn everything that was spawned for this screen
      .add_system_set(
        SystemSet::on_exit(GameState::Level2).with_system(despawn_screen::<OnGameScreen>),
      );
  }
}

pub fn init(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut state: ResMut<GameData>,
) {
  state.score = 0;
  state.camera_pos = Vec2::default();

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

  commands.insert_resource(EnemyAssets {
    atlas_handle: texture_atlas_handle_enemy,
    elite_atlas_handle: texture_atlas_handle_enemy2.clone(),
  });

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
    ((0.0, -map_size - map_size_25, 0.0), (map_size_50, map_size_50), (map_size_25, map_size_25)),
    ((0.0, map_size + map_size_25, 0.0), (map_size_50, map_size_50), (map_size_25, map_size_25)),
    ((-map_size - map_size_25, 0.0, 0.0), (map_size_50, map_size * 2.0), (map_size_25, map_size_25)),
    ((map_size + map_size_25, 0.0, 0.0), (map_size_50, map_size * 2.0), (map_size_25, map_size_25)),
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
      RigidBody::Fixed,
      CollisionGroups::new(Group::GROUP_6, Group::GROUP_1),
    ));
  }

  commands.spawn((
    OnGameScreen,
    SpriteBundle {
      texture: asset_server.load("cave.png"),
      transform: Transform {
        translation: Vec3::new(0., 0., 0.0),
        ..default()
      },
      ..default()
    },
    EnemySpawner {
      timer: Timer::from_seconds(0.3, TimerMode::Repeating),
      initial_delay: Timer::from_seconds(2.0, TimerMode::Once),
      spawn_count: 0,
      spawn_limit: 60,
      enemy_type: EnemySpawnerType::Normal,
    },
  ));


  commands.spawn((
    OnGameScreen,
    SpriteBundle {
      texture: asset_server.load("cave.png"),
      transform: Transform {
        translation: Vec3::new(400., -400., 0.0),
        ..default()
      },
      ..default()
    },
    EnemySpawner {
      timer: Timer::from_seconds(0.01, TimerMode::Repeating),
      initial_delay: Timer::from_seconds(25.0, TimerMode::Once),
      spawn_count: 0,
      spawn_limit: 32,
      enemy_type: EnemySpawnerType::Elite,
    },
  ));


  commands.spawn((
    OnGameScreen,
    SpriteBundle {
      texture: asset_server.load("cave.png"),
      transform: Transform {
        translation: Vec3::new(-400., -400., 0.0),
        ..default()
      },
      ..default()
    },
    EnemySpawner {
      timer: Timer::from_seconds(0.01, TimerMode::Repeating),
      initial_delay: Timer::from_seconds(25.0, TimerMode::Once),
      spawn_count: 0,
      spawn_limit: 32,
      enemy_type: EnemySpawnerType::Elite,
    },
  ));

  commands.spawn((
    OnGameScreen,
    SpriteBundle {
      texture: asset_server.load("cave.png"),
      transform: Transform {
        translation: Vec3::new(-400., 400., 0.0),
        ..default()
      },
      ..default()
    },
    EnemySpawner {
      timer: Timer::from_seconds(0.01, TimerMode::Repeating),
      initial_delay: Timer::from_seconds(25.0, TimerMode::Once),
      spawn_count: 0,
      spawn_limit: 32,
      enemy_type: EnemySpawnerType::Elite,
    },
  ));


  commands.spawn((
    OnGameScreen,
    SpriteBundle {
      texture: asset_server.load("cave.png"),
      transform: Transform {
        translation: Vec3::new(400., 400., 0.0),
        ..default()
      },
      ..default()
    },
    EnemySpawner {
      timer: Timer::from_seconds(0.01, TimerMode::Repeating),
      initial_delay: Timer::from_seconds(25.0, TimerMode::Once),
      spawn_count: 0,
      spawn_limit: 32,
      enemy_type: EnemySpawnerType::Elite,
    },
  ));

  commands
    .spawn((
      OnGameScreen,
      Player::One,
      Gun {
        cooldown: Timer::from_seconds(0.2, TimerMode::Repeating),
      },
      ActiveEvents::COLLISION_EVENTS,
      CollisionGroups::new(Group::GROUP_1, Group::GROUP_3.union(Group::GROUP_6)),
      SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, -100.0, 0.5)),
        ..default()
      },
      AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
      Velocity::default(),
      RigidBody::Dynamic,
    ))
    .insert((Collider::ball(10.), Restitution::coefficient(0.7)))
    .insert(InputManagerBundle::<Action> {
      action_state: ActionState::default(),
      input_map: InputMap::default()
        .insert(VirtualDPad::arrow_keys(), Action::Move)
        .insert(KeyCode::Space, Action::Attack)
        .build(),
    })
    .with_children(|parent| {
      parent.spawn((
        OnGameScreen,
        LightningGun {
          cooldown: Timer::from_seconds(6.0, TimerMode::Once),
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
    });

  commands
    .spawn((
      OnGameScreen,
      Player::Two,
      Gun {
        cooldown: Timer::from_seconds(0.2, TimerMode::Repeating),
      },
      ActiveEvents::COLLISION_EVENTS,
      CollisionGroups::new(Group::GROUP_1, Group::GROUP_3.union(Group::GROUP_6)),
      SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.5)),
        ..default()
      },
      AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
      RigidBody::Dynamic,
      Velocity::default(),
    ))
    .insert((Collider::ball(10.), Restitution::coefficient(0.7)))
    .insert(InputManagerBundle::<Action> {
      action_state: ActionState::default(),
      input_map: InputMap::default()
        .insert(VirtualDPad::wasd(), Action::Move)
        .insert(KeyCode::Q, Action::Attack)
        .build(),
    })
    .with_children(|parent| {
      parent.spawn((
        OnGameScreen,
        LinkGun {
          cooldown: Timer::from_seconds(8.0, TimerMode::Once),
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
    });
}

fn end_condition(
  players: Query<&Player>,
  enemies: Query<&Enemy>,
  spawners: Query<&EnemySpawner>,
  mut game_state: ResMut<State<GameState>>,
) {
  // game won
  if enemies.is_empty() && spawners.iter().all(|spawner| spawner.spawn_count >= spawner.spawn_limit) {
    game_state.set(GameState::LevelSelect).unwrap()
  }

  // game lost
  if players.is_empty() {
    game_state.set(GameState::LevelSelect).unwrap()
  }
}