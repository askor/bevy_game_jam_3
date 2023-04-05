mod game_manager;
pub(crate) mod level;
mod gameplay_elements;
mod level_test_env;
mod launcher;

use bevy::prelude::*;
use self::{game_manager::GameManagerPlugin, level_test_env::LevelTestEnvironmentPlugin, level::LevelPlugin, launcher::LauncherPlugin, gameplay_elements::GameplayElementsPlugin};

pub use launcher::LaunchEvent;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GameManagerPlugin)
            // .add_plugin(LevelTestEnvironmentPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(LauncherPlugin)
            .add_plugin(GameplayElementsPlugin)
        ;
    }
}