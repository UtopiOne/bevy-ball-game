mod camera;
mod enemy;
#[path = "resources/enemy_spawn_timer.rs"]
mod enemy_spawn_timer;
mod events;
mod player;
#[path = "resources/score.rs"]
mod score;
#[path = "resources/star_spawn_timer.rs"]
mod star_spawn_timer;
mod stars;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemiesPlugin)
        .add_plugins(stars::StarsPlugin)
        .add_plugins(score::ScorePlugin)
        .add_plugins(star_spawn_timer::StarSpawnTimerPlugin)
        .add_plugins(enemy_spawn_timer::EnemySpawnTimerPlugin)
        .add_plugins(events::EventsPlugin)
        .run();
}


