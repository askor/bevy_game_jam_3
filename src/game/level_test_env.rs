use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::AppState;
use super::gameplay_elements::{Goal, Box};
use super::gameplay_elements::ball::GolfBall;
use crate::game::level::Level;

pub struct LevelTestEnvironmentPlugin;

impl Plugin for LevelTestEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup
                .in_schedule(OnEnter(AppState::Playing))
            );
    }
}

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let level = commands.spawn((
        SpatialBundle::default(),
        Level,
        Name::new("Level"),
    )).id();

    // Ball
    let ball = commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::try_from(shape::Icosphere{radius: 1., subdivisions: 5 }).unwrap()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 10., 0.0),
        ..default()
        },
        Collider::ball(1.),
        Restitution::new(1.),
        RigidBody::Dynamic,
        LockedAxes::all(),
        // GravityScale(0.0),
        // LaunchTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
        GolfBall,
        Name::new("Ball"),
    )).id();

    // Goal
    let goals_dims = create_physical_box(2., 2., 2.);
    let goal = commands.spawn((PbrBundle {
        mesh: meshes.add(goals_dims.1),
        material: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
        transform: Transform::from_xyz(0.0, 0.0, -40.0),
        ..default()
        },
        goals_dims.0,
        RigidBody::Fixed,
        Restitution::new(1.0),
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        Goal,
        Name::new("Goal"),
    )).id();

    // Ground
    let ground_dims = create_physical_box(100., 1., 100.);
    let ground = commands.spawn((PbrBundle {
        mesh: meshes.add(ground_dims.1),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, -4., 0.0),
        ..default()
        },
        ground_dims.0, // Collider
        ground_dims.2, // Box
        RigidBody::Fixed,
        Restitution::new(1.0)
    )).id();

    // Add as child of level to save
    commands.entity(level).add_child(ball);
    commands.entity(level).add_child(goal);
    commands.entity(level).add_child(ground);

    // light (not saved)
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn create_physical_box(x: f32, y: f32, z: f32) -> (Collider, Mesh, Box) {
    let collider = Collider::cuboid(x/2., y/2., z/2.);
    let mesh = Mesh::from(shape::Box::new(x, y, z));
    let box_dims = Box {x, y, z};

    return (collider, mesh, box_dims);
}