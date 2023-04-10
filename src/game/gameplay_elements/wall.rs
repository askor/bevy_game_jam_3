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
            .register_type::<LowGravWall>()
            .add_system(plain_wall_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(bounce_wall_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(bounce_wall_collision)
            .add_system(low_grav_wall_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(low_grav_wall_collision)
            .add_system(disable_low_grav)
            ;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Box {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl Default for Box {
    fn default() -> Self {
        Self { x: 1.0, y: 20., z: 20. }
    }
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
            // Restitution::new(1.0),
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

fn bounce_wall_collision(
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

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct LowGravWall;

pub(crate) fn low_grav_wall_added(
    query: Query<(Entity, &Box, &Transform), Added<LowGravWall>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, box_dims, transform) in query.iter() {
        info!("LowGrav added");
        let ground_dims = create_physical_box(box_dims.x, box_dims.y, box_dims.z);
        commands.entity(entity).insert((
            meshes.add(ground_dims.1),
            materials.add(StandardMaterial {
                emissive: Color::rgb_linear(1.0, 1.0, 6.99),
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
            Name::new("LowGrav wall"),
        ));
    }
}

#[derive(Component)]
struct GravityEffectTimer(Timer);

fn low_grav_wall_collision(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    q_wall: Query<Entity, With<LowGravWall>>,
    q_ball: Query<Entity, With<GolfBall>>,
) {
    for collision in collisions.iter() {
        let grav_scale = 0.1;
        let low_grav_time = 10.0;
        
        match collision {
            CollisionEvent::Stopped(a, b, _) => {
                if q_wall.get(*a).is_ok() || q_wall.get(*b).is_ok() {
                    info!("LowGrav!");
                    let ball = q_ball.single();
                    commands.entity(ball).insert(GravityScale(grav_scale));
                    commands.entity(ball).insert(GravityEffectTimer(Timer::from_seconds(low_grav_time, TimerMode::Once)));
                }
            },
            CollisionEvent::Started(_, _, _) => (),
        }
    }
}

fn disable_low_grav(
    mut commands: Commands,
    mut q_ball: Query<(Entity, &mut GravityEffectTimer), With<GolfBall>>,
    time: Res<Time>,
) {
    for (ball, mut timer) in q_ball.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(ball).insert(GravityScale(1.0));
            commands.entity(ball).remove::<GravityEffectTimer>();
            info!("Removed low grav!")
        }
    }
}