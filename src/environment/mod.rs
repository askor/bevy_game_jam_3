mod test_env;
mod physics;
mod physics_test_env;

use bevy::prelude::*;
use test_env::TestEnvironmentPlugin;

use self::{physics::PhysicsPlugin, physics_test_env::PhysicsTestEnvironmentPlugin};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(TestEnvironmentPlugin)
            // .add_plugin(PhysicsTestEnvironmentPlugin)
            .add_plugin(PhysicsPlugin);
    }
}