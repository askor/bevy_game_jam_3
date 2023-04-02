mod test_env;
mod physics;

use bevy::prelude::*;
use test_env::TestEnvironmentPlugin;

use self::physics::PhysicsPlugin;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TestEnvironmentPlugin)
            .add_plugin(PhysicsPlugin);
    }
}