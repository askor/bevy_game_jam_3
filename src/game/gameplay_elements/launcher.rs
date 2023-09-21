use bevy::prelude::*;
use bevy_rapier3d::prelude::{LockedAxes, Velocity};
use leafwing_input_manager::prelude::*;

use crate::camera::{free_cam, Focus};
use crate::game::level::Level;
use crate::{actions::Action, game::game_manager::GameState, AppState, loading::AudioAssets};

use super::create_physical_box;
use super::ball::{GolfBallBundle, BallState};
use super::ball::GolfBall;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LaunchEvent>()
            .register_type::<Launcher>()
            .insert_resource(LaunchVelocity(50.0))
            .add_system(launcher_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(ball_stopped
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(play_launch_sound.run_if(on_event::<LaunchEvent>()))
            .add_system(launch_ball.run_if(not(free_cam)))
            .add_system(aim_launcher
                .run_if(not(free_cam))
                // .in_set(OnUpdate(BallState::Aiming))
            )
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
                transform: Transform::from_translation(transform.translation),
                ..default()
            },
            InputManagerBundle {
                action_state: ActionState::default(),
                input_map: InputMap::default()
                    .insert(DualAxis::left_stick(), Action::Aim)
                    // .insert(VirtualDPad::wasd(), Action::Aim)
                    .insert(VirtualDPad::arrow_keys(), Action::Aim)
                    .insert(KeyCode::Space, Action::Shoot)
                    .insert(GamepadButtonType::South, Action::Shoot)
                    .build(),
            },
            Focus,
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
    let sensitivity = 1.0;
    if let Ok((mut trans, action_state)) = query.get_single_mut() { 
        let axis_pair = action_state.clamped_axis_pair(Action::Aim).unwrap();

        rotation.y = sensitivity * axis_pair.y() * time.delta_seconds() + rotation.y;
        rotation.x = sensitivity * -axis_pair.x() * time.delta_seconds() + rotation.x;

        trans.rotation = Quat::from_rotation_y(rotation.x) * Quat::from_rotation_x(rotation.y);
    }
}

fn launch_ball(
    mut commands: Commands,
    launcher_q: Query<(&Transform, &ActionState<Action>, Entity), With<Launcher>>,
    ball_q: Query<Entity, With<GolfBall>>,
    launc_vel: Res<LaunchVelocity>,
    mut launch_event: EventWriter<LaunchEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ball_state: ResMut<NextState<BallState>>,
) {
    if let Ok((launcher_trans, action_state, launcher_entity)) = launcher_q.get_single() {
        if action_state.just_released(Action::Shoot) {
            // Despawn other balls
            for ball in ball_q.iter() { commands.entity(ball).despawn_recursive(); }

            // Spawn ball with physics
            let ball = commands.spawn(GolfBallBundle {
                pbr: PbrBundle {
                    mesh: meshes.add(Mesh::try_from(shape::Icosphere{radius: 1., subdivisions: 5 }).unwrap()),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: launcher_trans.clone(),
                    ..default()
                },
                ..default()
            }).insert(Focus)
            .id();

            let velocity = launcher_trans.forward() * launc_vel.0 * action_state.previous_duration(Action::Shoot).as_secs_f32().min(2.0);
        
            commands.entity(ball).insert( Velocity{ linvel: velocity, angvel: Vec3::ZERO });
            launch_event.send(LaunchEvent);
            ball_state.set(BallState::InPlay);

            commands.entity(launcher_entity).remove::<Focus>();

            info!("Launch!");
            // Free ball axes
            // let mut axes = q_locked_axes.get_mut(entity).unwrap();
            // axes.toggle(LockedAxes::all());
        }
    }
}

// TODO Fix me (add list of last positions, or check accel or vel?)
fn ball_stopped (
    mut commands: Commands,
    ball_q: Query<(Entity, &Transform), With<GolfBall>>,
    launcher_q: Query<Entity, With<Launcher>>,
    level_q: Query<Entity, With<Level>>,
    mut last_pos: Local<Vec3>,
    mut ball_state: ResMut<NextState<BallState>>,
) {
    for (ball, transform) in ball_q.iter() {
        // info!("Ball stopped!");
        if transform.translation.distance_squared(*last_pos) < 0.00001 {
            info!("Stopped ball!");

            for entity in launcher_q.iter() {
                commands.entity(entity).despawn_recursive();
            }
            let level = level_q.single();
            let launcher = commands.spawn((
                Launcher,
                SpatialBundle { transform: *transform, ..default() },
            )).id();
            ball_state.set(BallState::Aiming);
            commands.entity(level).add_child(launcher);
            commands.entity(ball).despawn_recursive();
        }
        *last_pos = transform.translation;
    }
}


fn play_launch_sound(
    assets: Res<AudioAssets>,
    audio: Res<Audio>,
    // mut sound_index: Local<u8>,
) {
    let sound = assets.launch1.clone();
    // let sound_count = 4;

    // if *sound_index == 0 {
    //     sound = assets.launch1.clone();
    // }
    // else if *sound_index == 1 {
    //     sound = assets.launch2.clone();
    // }
    // else if *sound_index == 2 {
    //     sound = assets.launch3.clone();
    // }
    // else if *sound_index == 3 {
    //     sound = assets.launch4.clone();
    // }

    audio.play(sound);

    // Update index for new sound
    // *sound_index += 1;
    // if *sound_index > (sound_count -1) {
    //     *sound_index = 1u8;
    // }
}





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
