use bevy::prelude::*;
use bevy_rapier3d::prelude::{LockedAxes, Velocity};

use super::GolfBall;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LaunchEvent>()
            .add_system(launch_ball.run_if(on_event::<LaunchEvent>()))
            .add_system(launch_countdown);
    }
}

pub struct LaunchEvent;

#[derive(Component)]
pub(crate) struct Launcher;

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
