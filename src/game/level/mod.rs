mod level_manager;

use bevy::prelude::*;

use self::level_manager::LevelManagerPlugin;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LevelManagerPlugin);
    }
}

#[derive(Component)]
struct Level;