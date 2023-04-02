use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::GameState;

pub struct PhysicsTestEnvironmentPlugin;

impl Plugin for PhysicsTestEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let box_dims = create_physical_box(100., 1., 100.);
    
    // Ground
    commands.spawn((PbrBundle {
        mesh: meshes.add(box_dims.1),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, -4., 0.0),
        ..default()
        },
        box_dims.0,
        RigidBody::Fixed,
        Restitution::new(1.0)
    ));

    // Ball
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::try_from(shape::Icosphere{radius: 1., subdivisions: 5 }).unwrap()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 10., 0.0),
        ..default()
        },
        Collider::ball(1.),
        Restitution::new(1.),
        RigidBody::Dynamic,
    ));

    let box_dims = create_physical_box(100., 1., 100.);
    
    // cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(box_dims.1),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, -4., 0.0),
        ..default()
        },
        box_dims.0,
        RigidBody::Fixed,
        Restitution::new(1.0)
    ));

    // light
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

fn create_physical_box(x: f32, y: f32, z: f32) -> (Collider, Mesh) {
    let collider = Collider::cuboid(x/2., y/2., z/2.);
    let mesh = Mesh::from(shape::Box::new(x, y, z));

    return (collider, mesh);
}