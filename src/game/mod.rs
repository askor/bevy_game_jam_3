mod game_manager;
mod level;
mod gameplay_elements;
mod level_test_env;
mod launcher;

// pub use gameplay_elements::*;

use bevy::prelude::*;
use self::{game_manager::GameManagerPlugin, level_test_env::LevelTestEnvironmentPlugin, level::LevelPlugin, launcher::{LaunchTimer, LauncherPlugin}};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GameManagerPlugin)
            .add_plugin(LevelTestEnvironmentPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(LauncherPlugin)
        ;
    }
}