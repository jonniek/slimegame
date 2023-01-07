use bevy::prelude::*;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub struct Killzone;

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
