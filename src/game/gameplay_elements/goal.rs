use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{game::GameState, AppState};

use super::create_physical_box;

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Goal>() // TODO remove?
            .add_system(goal_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
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
            materials.add(StandardMaterial {
                emissive: Color::rgb_linear(1.0, 10.0, 1.0),
                ..default()
            }),
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