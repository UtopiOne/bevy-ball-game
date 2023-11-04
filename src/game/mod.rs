pub mod enemy;
mod player;
mod score;
pub mod star;
mod systems;

use bevy::prelude::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::{events::GameOver, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            // Events
            .add_event::<GameOver>()
            // Plugins
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            // Systems
            .add_systems(
                Update,
                (toggle_simulation.run_if(in_state(AppState::Game)),),
            );
    }
}

#[derive(States, Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
