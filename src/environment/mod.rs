mod test_env;
mod physics;
mod physics_test_env;
mod lighting;

use bevy::prelude::*;
// use test_env::TestEnvironmentPlugin;
// use self::physics_test_env::PhysicsTestEnvironmentPlugin;

use self::{physics::PhysicsPlugin, lighting::LightingPlugin};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(TestEnvironmentPlugin)
            // .add_plugin(PhysicsTestEnvironmentPlugin)
            .add_plugin(LightingPlugin)
            .add_plugin(PhysicsPlugin);
    }
}