use bevy::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 3.0;

pub struct EnemySpawnTimerPlugin;

impl Plugin for EnemySpawnTimerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>();
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
