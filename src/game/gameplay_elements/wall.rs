use crate::{game::GameState, AppState};
use super::{create_physical_box, ball::GolfBall};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Box>()
            .register_type::<PlainWall>()
            .register_type::<BounceWall>()
            .add_system(plain_wall_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(bounce_wall_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(wall_collision)
            ;
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct Box {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct PlainWall;

// On Box added
pub(crate) fn plain_wall_added(
    query: Query<(Entity, &Box, &Transform), Added<PlainWall>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, box_dims, transform) in query.iter() {
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
            Restitution::new(1.0),
            Name::new("Plain wall"),
        ));
    }
}



#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct BounceWall;

pub(crate) fn bounce_wall_added(
    query: Query<(Entity, &Box, &Transform), Added<BounceWall>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, box_dims, transform) in query.iter() {
        info!("Bounce added");
        let ground_dims = create_physical_box(box_dims.x, box_dims.y, box_dims.z);
        commands.entity(entity).insert((
            meshes.add(ground_dims.1),
            materials.add(StandardMaterial {
                emissive: Color::rgb_linear(1.0, 6.99, 1.0),
                ..default()
            }),
            SpatialBundle {
                transform: *transform,
                ..default()
            },
            ground_dims.0, // Collider
            ground_dims.2, // Box
            RigidBody::Fixed,
            Restitution::new(1.0),
            ActiveEvents::COLLISION_EVENTS,
            Name::new("Bounce wall"),
        ));
    }
}

fn wall_collision(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    q_wall: Query<Entity, With<BounceWall>>,
    q_ball: Query<Entity, With<GolfBall>>,
) {
    for collision in collisions.iter() {
        let bounce_strength = 100.;
        
        match collision {
            CollisionEvent::Stopped(a, b, _) => {
                if q_wall.get(*a).is_ok() || q_wall.get(*b).is_ok() {
                    info!("Bounce!");
                    let ball = q_ball.single();
                    commands.entity(ball).insert(ExternalImpulse {impulse: Vec3::Y * bounce_strength, ..default() });
                }
            },
            CollisionEvent::Started(_, _, _) => (),
        }
    }
}