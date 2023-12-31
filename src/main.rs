mod events;
mod systems;

mod game;
mod main_menu;

use bevy::prelude::*;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // My plugins
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        // Systems
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (
                exit_game,
                handle_game_over,
                transition_to_game_state,
                transition_to_main_menu_state,
            ),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
