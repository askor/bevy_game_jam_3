use std::f32::consts::PI;

use bevy::prelude::*;
use smooth_bevy_cameras::{LookTransformBundle, LookTransform, Smoother, LookTransformPlugin, LookAngles};

use crate::{AppState, game::gameplay_elements::{LaunchEvent}};
use crate::game::gameplay_elements::ball::GolfBall;

pub struct InternalCameraPlugin;

impl Plugin for InternalCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LookTransformPlugin)
            .add_system(setup.in_schedule(OnEnter(AppState::Playing)))
            .add_system(ball_follow_camera
                .in_set(OnUpdate(AppState::Playing))
            )
            ;
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup(
    mut commands: Commands,
) {
    let eye = Vec3::new(0., 2., 10.);
    let target = Vec3::new(0., 0., 0.);

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2., 10.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            ..default()
        },
        LookTransformBundle {
            transform: LookTransform::new(eye, target, Vec3::Y),
            smoother: Smoother::new(0.9),
        },
        Name::new("Camera"),
        MainCamera,
    ));
}

fn ball_follow_camera(
    ball_q: Query<&Transform, With<GolfBall>>,
    mut camera_q: Query<&mut LookTransform, With<MainCamera>>,
) {
    if let Ok(ball_trans) = ball_q.get_single() {
        if let Ok(mut look) = camera_q.get_single_mut() {
            
            let delta = Vec2::new(0., PI / 8.);
            let mut angles = LookAngles::from_vector(look.look_direction().unwrap());
            angles.set_pitch(delta.y);
            angles.set_yaw(delta.x);

            let offset = look.radius().min(35.);

            // Third-person.
            look.eye = look.target + offset * angles.unit_vector();

            look.target = ball_trans.translation;
        }
    }
}