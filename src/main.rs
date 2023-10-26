mod camera;
mod enemy;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemiesPlugin)
        .run();
}
