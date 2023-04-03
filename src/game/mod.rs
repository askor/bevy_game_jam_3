mod game_manager;
mod gameplay_elements;
mod level_test_env;

// pub use gameplay_elements::*;

use bevy::prelude::*;
use self::{game_manager::GameManagerPlugin, level_test_env::LevelTestEnvironmentPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GameManagerPlugin)
            .add_plugin(LevelTestEnvironmentPlugin)
        ;
    }
}