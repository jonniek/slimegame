use bevy::prelude::*;

#[derive(Component)]
pub enum Player {
  One,
  Two,
}

#[derive(Component)]
pub struct EnemySpawner {
  pub timer: Timer,
}

#[derive(Component)]
pub struct EnemyEliteSpawner {
  pub timer: Timer,
}

#[derive(Component)]
pub struct Homing;

#[derive(Component)]
pub struct Killzone;

#[derive(Resource, Default)]
pub struct EnemyAssets {
  pub atlas_handle: Handle<TextureAtlas>,
  pub elite_atlas_handle: Handle<TextureAtlas>,
}

#[derive(Component, Debug)]
pub struct Enemy {
  pub direction: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Debug)]
pub struct Projectile {
  pub damage: f32,
}

#[derive(Component, Debug)]
pub struct Gun {
  pub cooldown: Timer,
}

#[derive(Component, Debug)]
pub struct LightningGun {
  pub cooldown: Timer,
}

#[derive(Component, Debug)]
pub struct LinkGun {
  pub cooldown: Timer,
}

#[derive(Component, Debug)]
pub struct Link {
  pub damage: f32,
}

#[derive(Component)]
pub struct ExpirationTimer(pub Timer);

#[derive(Component, Debug)]
pub struct Health {
  pub current_health: f32,
  pub max_health: f32,
  pub dmg_timer: Timer,
}

impl Health {
  pub fn new(health: f32) -> Self {
    Health {
      current_health: health,
      max_health: health,
      dmg_timer: {
        let mut t = Timer::from_seconds(0.15, TimerMode::Once);
        t.pause();
        t
      },
    }
  }
}
