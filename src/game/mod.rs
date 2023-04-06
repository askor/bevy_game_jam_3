mod game_manager;
pub(crate) mod level;
pub mod gameplay_elements;
mod level_test_env;

use bevy::prelude::*;
use self::{game_manager::GameManagerPlugin, level_test_env::LevelTestEnvironmentPlugin, level::LevelPlugin, gameplay_elements::GameplayElementsPlugin};
pub use self::game_manager::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GameManagerPlugin)
            // .add_plugin(LevelTestEnvironmentPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(GameplayElementsPlugin)
        ;
    }
}