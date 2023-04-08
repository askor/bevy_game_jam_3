use super::create_physical_box;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct Box {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

// On Box added
pub(crate) fn box_added(
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
