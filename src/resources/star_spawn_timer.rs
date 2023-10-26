use bevy::prelude::*;

pub const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarSpawnTimerPlugin;

impl Plugin for StarSpawnTimerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>();
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
