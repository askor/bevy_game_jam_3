use bevy::prelude::*;
use bevy_rapier3d::prelude::{LockedAxes, Velocity};
use leafwing_input_manager::prelude::*;

use crate::{actions::Action, game::game_manager::GameState, AppState};

use super::{GolfBall, create_physical_box};

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LaunchEvent>()
            .register_type::<Launcher>()
            .insert_resource(LaunchVelocity(10.0))
            .add_system(launcher_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(launch_ball)
            .add_system(aim_launcher)
            .add_system(launch_countdown);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct Launcher;

#[derive(Resource)]
pub struct LaunchVelocity(pub f32);

pub struct LaunchEvent;

// On launcher added
fn launcher_added(
    query: Query<(Entity, &Transform), Added<Launcher>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((entity, transform)) = query.get_single() {
        let box_dims = create_physical_box(1., 1., 3.);

        commands.entity(entity).insert((
            meshes.add(Mesh::try_from(box_dims.1).unwrap()),
            materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            SpatialBundle {
                transform: *transform,
                ..default()
            },
            InputManagerBundle {
                action_state: ActionState::default(),
                input_map: InputMap::default()
                    .insert(DualAxis::left_stick(), Action::Aim)
                    // .insert(VirtualDPad::wasd(), Action::Aim)
                    .insert(VirtualDPad::arrow_keys(), Action::Aim)
                    .insert(KeyCode::Space, Action::Shoot)
                    .build(),
            },
            Launcher,
            Name::new("Launcher"),
        ));
    }
}

fn aim_launcher(
    mut query: Query<(&mut Transform, &ActionState<Action>), With<Launcher>>,
    time: Res<Time>,
    mut rotation: Local<Vec2>,
) {
    if let Ok((mut trans, action_state)) = query.get_single_mut() { 
        if action_state.pressed(Action::Aim) {
            let axis_pair = action_state.clamped_axis_pair(Action::Aim).unwrap();
            // info!("AIM: {:?}", axis_pair);

            let sensitivity = 1.0;

            // let mut pitch = trans.rotation.xyz().y;
            // let mut yaw = trans.rotation.xyz().x;

            rotation.y = sensitivity * axis_pair.y() * time.delta_seconds() + rotation.y;
            rotation.x = sensitivity * -axis_pair.x() * time.delta_seconds() + rotation.x;

            trans.rotation = Quat::from_rotation_y(rotation.x) * Quat::from_rotation_x(rotation.y);
        }
    }
}

fn launch_ball(
    mut commands: Commands,
    launcher_q: Query<(&Transform, &ActionState<Action>), With<Launcher>>,
    mut ball_q: Query<(Entity, &mut Transform), (With<GolfBall>, Without<Launcher>)>,
    mut q_locked_axes: Query<&mut LockedAxes>,
    launc_vel: Res<LaunchVelocity>,
) {
    if let Ok((launcher_trans, action_state)) = launcher_q.get_single() { 
        if let Ok((entity, mut ball_trans)) = ball_q.get_single_mut() {
            if !action_state.just_pressed(Action::Shoot) { return; }
            info!("Launch!");
            // Free ball axes
            // let mut axes = q_locked_axes.get_mut(entity).unwrap();
            // axes.toggle(LockedAxes::all());
            // Move to launcher
            ball_trans.translation = launcher_trans.translation;
            // Add velocity
            let velocity = launcher_trans.forward() * launc_vel.0;
            commands.entity(entity).insert( Velocity{ linvel: velocity, angvel: Vec3::ZERO });
        }
    }
}


/////////////////////////////////////////////////////////

#[derive(Component, Deref, DerefMut)]
pub(crate) struct LaunchTimer(pub Timer);

// Launch ball after one sec
fn launch_countdown(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut LaunchTimer)>,
    mut q_locked_axes: Query<&mut LockedAxes>,
) {
    for (entity, mut timer) in &mut query {
        if timer.tick(time.delta()).finished() {
            info!("Launch!");
            let mut axes = q_locked_axes.get_mut(entity).unwrap();
            axes.toggle(LockedAxes::all());
            commands.entity(entity).remove::<LaunchTimer>();
            // commands.entity(entity).insert(LockedAxes::empty());
            commands.entity(entity).insert( Velocity{ linvel: Vec3::new(0., 0., -10.), angvel: Vec3::ZERO });
        }
    }
}