use super::create_physical_box;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct DeathZone {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

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
        Restitution::new(1.0)
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