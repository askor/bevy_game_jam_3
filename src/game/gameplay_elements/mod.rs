pub mod launcher;
pub mod ball;
pub mod wall;
pub mod death_zone;

use std::default;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::AppState;
use super::game_manager::GameState;

use self::{launcher::Launcher, ball::GolfBallPlugin, death_zone::{DeathZone, cleanup_death_zone, add_death_zone}};
pub use self::launcher::{LauncherPlugin, LaunchEvent};

pub struct GameplayElementsPlugin;

impl Plugin for GameplayElementsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LauncherPlugin)
            .add_plugin(GolfBallPlugin)
            .register_type::<Goal>() // TODO remove?
            .register_type::<wall::Box>()
            .register_type::<DeathZone>()
            .add_system(goal_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(wall::box_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(add_death_zone.in_schedule(OnEnter(AppState::Playing)))
            .add_system(cleanup_death_zone.in_schedule(OnExit(AppState::Playing)))
            ;
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct Goal;


// On Goal added
fn goal_added(
    query: Query<(Entity, &Transform), Added<Goal>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((entity, transform)) = query.get_single() {
        let goals_dims = create_physical_box(2., 2., 2.);
        commands.entity(entity).insert((
            meshes.add(goals_dims.1),
            materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
            SpatialBundle {
                transform: *transform,
                ..default()
            },
            goals_dims.0,
            RigidBody::Fixed,
            Restitution::new(1.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Name::new("Goal"),
        ));
    }
}


/////////////////////////////////////////////////////////////////

fn create_physical_box(x: f32, y: f32, z: f32) -> (Collider, Mesh, wall::Box) {
    let collider = Collider::cuboid(x/2., y/2., z/2.);
    let mesh = Mesh::from(shape::Box::new(x, y, z));
    let box_dims = wall::Box {x, y, z};

    return (collider, mesh, box_dims);
}
