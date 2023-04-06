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
            .add_system(launcher_added
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(launch_ball.run_if(on_event::<LaunchEvent>()))
            .add_system(aim_launcher)
            .add_system(launch_countdown);
    }
}

pub struct LaunchEvent;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct Launcher;

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
    mut query: Query<Entity, With<GolfBall>>,
    mut q_locked_axes: Query<&mut LockedAxes>,
) {
    for entity in &mut query {
        info!("Launch!");
        let mut axes = q_locked_axes.get_mut(entity).unwrap();
        axes.toggle(LockedAxes::all());
        commands.entity(entity).insert( Velocity{ linvel: Vec3::new(0., 0., -10.), angvel: Vec3::ZERO });
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