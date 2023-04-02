use bevy::prelude::*;

use self::test_env::TestEnvironmentPlugin;

mod test_env;
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TestEnvironmentPlugin);
    }
}