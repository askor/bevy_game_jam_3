use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::GameState;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_system(print_sensor_collison.in_set(OnUpdate(GameState::Playing)))
            ;
    }
}

fn print_sensor_collison(
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.iter() {
        info!("Collision: {:?}", collision);
    }
}