pub mod launcher;
pub mod ball;

use std::default;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::AppState;
use super::game_manager::GameState;

use self::{launcher::Launcher, ball::GolfBallPlugin};
pub use self::launcher::{LauncherPlugin, LaunchEvent};

pub struct GameplayElementsPlugin;

impl Plugin for GameplayElementsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LauncherPlugin)
            .add_plugin(GolfBallPlugin)
            .register_type::<Goal>() // TODO remove?
            .register_type::<Box>()
            .add_system(goal_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(box_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            ;
    }
}


// mod ball {
//     use crate::AppState::crate;

//     #[derive(Component, Reflect, Default)]
//     #[reflect(Component)]
//     pub(crate) struct GolfBall;

//     #[derive(Bundle)]
//     pub(crate) struct GolfBallBundle {
//         #[bundle]
//         pub(crate) pbr: PbrBundle,
//         pub(crate) name: Name,
//         pub(crate) collider: Collider,
//         pub(crate) restitution: Restitution,
//         pub(crate) rigidbody: RigidBody,
//         pub(crate) golf_ball: GolfBall,
//     }

//     impl Default for GolfBallBundle {
//         fn default() -> Self {
//             Self {
//                 pbr: PbrBundle::default(),
//                 name: Name::new("Golf ball"),
//                 collider: Collider::ball(1.),
//                 restitution: Restitution::new(1.),
//                 rigidbody: RigidBody::Dynamic,
//                 golf_ball: GolfBall, 
//             }
//         }
//     }
// }

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct Goal;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct Box {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

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

// On Box added
fn box_added(
    query: Query<(Entity, &Box, &Transform), Added<Box>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((entity, box_dims, transform)) = query.get_single() {
        let ground_dims = create_physical_box(box_dims.x, box_dims.y, box_dims.z);
        commands.entity(entity).insert((
            meshes.add(ground_dims.1),
            materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            SpatialBundle {
                transform: *transform,
                ..default()
            },
            ground_dims.0, // Collider
            ground_dims.2, // Box
            RigidBody::Fixed,
            Restitution::new(1.0)
        ));
    }
}


/////////////////////////////////////////////////////////////////

fn create_physical_box(x: f32, y: f32, z: f32) -> (Collider, Mesh, Box) {
    let collider = Collider::cuboid(x/2., y/2., z/2.);
    let mesh = Mesh::from(shape::Box::new(x, y, z));
    let box_dims = Box {x, y, z};

    return (collider, mesh, box_dims);
}
