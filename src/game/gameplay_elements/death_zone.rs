use super::{create_physical_box, ball::GolfBall};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct DeathZonePlugin;

impl Plugin for DeathZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(death_zone_collision);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct DeathZone;

// On death zone added
pub(crate) fn add_death_zone(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let span = 100000.0;
    let ground_dims = create_physical_box(span, 4.0, span);

    commands.spawn((
        meshes.add(ground_dims.1),
        materials.add(StandardMaterial {
            emissive: Color::rgb_linear(6.99, 1.0, 1.0),
            ..default()
        }),
        SpatialBundle {
            transform: Transform::from_xyz(0., -8., 0.),
            ..default()
        },
        ground_dims.0, // Collider
        RigidBody::Fixed,
        Restitution::new(1.0),
        ActiveEvents::COLLISION_EVENTS,
        Name::new("Death Zone"),
        DeathZone,
    ));
}

pub(crate) fn cleanup_death_zone(
    mut commands: Commands,
    query: Query<Entity, With<DeathZone>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn death_zone_collision(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    q_death_zone: Query<Entity, With<DeathZone>>,
    q_ball: Query<Entity, With<GolfBall>>,
) {
    for collision in collisions.iter() {
        info!("Death collisioon!");
        
        match collision {
            CollisionEvent::Started(a, b, _) => {
                if q_death_zone.get(*a).is_ok() || q_death_zone.get(*b).is_ok() {
                    info!("Dead!");
                    let ball = q_ball.single();
                    commands.entity(ball).despawn_recursive();
                }
            },
            CollisionEvent::Stopped(_, _, _) => (),
        }
    }
}