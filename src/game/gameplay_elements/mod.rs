pub mod launcher;
pub mod ball;
pub mod goal;
pub mod wall;
pub mod death_zone;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::AppState;

use self::{ball::GolfBallPlugin, death_zone::{DeathZone, cleanup_death_zone, add_death_zone, DeathZonePlugin}, wall::WallPlugin, goal::GoalPlugin};
pub use self::launcher::{LauncherPlugin, LaunchEvent};

pub struct GameplayElementsPlugin;

impl Plugin for GameplayElementsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LauncherPlugin)
            .add_plugin(GolfBallPlugin)
            .add_plugin(GoalPlugin)
            .add_plugin(WallPlugin)
            .add_plugin(DeathZonePlugin)
            .register_type::<DeathZone>()
            .add_system(add_death_zone.in_schedule(OnEnter(AppState::Playing)))
            .add_system(cleanup_death_zone.in_schedule(OnExit(AppState::Playing)))
            ;
    }
}


/////////////////////////////////////////////////////////////////

fn create_physical_box(x: f32, y: f32, z: f32) -> (Collider, Mesh, wall::Box) {
    let collider = Collider::cuboid(x/2., y/2., z/2.);
    let mesh = Mesh::from(shape::Box::new(x, y, z));
    let box_dims = wall::Box {x, y, z};

    return (collider, mesh, box_dims);
}
