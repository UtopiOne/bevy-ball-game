use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub const INITIAL_NUMBER_OF_STARS: u32 = 5;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_star)
            .add_systems(Update, (spawn_stars_over_time, tick_star_spawn_timer));
    }
}
