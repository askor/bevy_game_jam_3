use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{game::GameState, AppState};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum BallState {
    Aiming,
    InPlay,
    Hole,
    Dead,
    #[default]
    Disabled,
}

pub struct GolfBallPlugin;

impl Plugin for GolfBallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<BallState>()
            .register_type::<GolfBall>()
            .add_system(golfball_added
                // .in_set(OnUpdate(GameState::InProgress))
                // .run_if(not(in_state(BallState::Disabled)))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(clean_balls.in_schedule(OnExit(GameState::Complete)))
            // .add_system(clean_balls.in_schedule(OnEnter(GameState::InProgress)))
            ;
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct GolfBall;

#[derive(Bundle)]
pub(crate) struct GolfBallBundle {
    #[bundle]
    pub(crate) pbr: PbrBundle,
    pub(crate) name: Name,
    pub(crate) collider: Collider,
    pub(crate) restitution: Restitution,
    pub(crate) rigidbody: RigidBody,
    pub(crate) golf_ball: GolfBall,
}

impl Default for GolfBallBundle {
    fn default() -> Self {
        Self {
            pbr: PbrBundle::default(),
            name: Name::new("Golf ball"),
            collider: Collider::ball(1.),
            restitution: Restitution::new(1.),
            rigidbody: RigidBody::Dynamic,
            golf_ball: GolfBall, 
        }
    }
}

// On golfball added
fn golfball_added(
    query: Query<(Entity, &Transform), Added<GolfBall>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, transform) in query.iter() {
        commands.entity(entity).insert((
            meshes.add(Mesh::try_from(shape::Icosphere{radius: 1., subdivisions: 5 }).unwrap()),
            materials.add(StandardMaterial {
                emissive: Color::rgb_linear(1.0, 1.0, 10.0),
                ..default()
            }),
            PointLightBundle {
                point_light: PointLight {
                    shadows_enabled: false,
                    ..default()
                },
                transform: *transform,
                ..default()
            },
            Collider::ball(1.),
            Damping { linear_damping: 0.4, angular_damping: 0.4 },
            // Restitution::new(1.),
            RigidBody::Dynamic,
            // LockedAxes::all(),
            // GravityScale(0.0),
            // LaunchTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            Name::new("Ball"),
        ));
    }
}

fn clean_balls (
    mut commands: Commands,
    ball_q: Query<Entity, With<GolfBall>>,
) {
    for ball_e in ball_q.iter() {
        commands.entity(ball_e).despawn_recursive();
    }
}