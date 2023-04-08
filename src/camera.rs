use std::f32::consts::PI;

use bevy::{prelude::*, core_pipeline::bloom::BloomSettings};
use smooth_bevy_cameras::{LookTransformBundle, LookTransform, Smoother, LookTransformPlugin, LookAngles};

use crate::{AppState, game::{gameplay_elements::{LaunchEvent, launcher::Launcher}, GameState}};
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
            .add_system(ball_follow_camera
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(reset_camera
                .in_set(OnUpdate(AppState::Playing))
                .in_set(OnUpdate(GameState::InProgress))
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
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0., 2., 10.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
        LookTransformBundle {
            transform: LookTransform::new(eye, target, Vec3::Y),
            smoother: Smoother::new(0.9),
        },
        Name::new("Camera"),
        MainCamera,
    ));
}

fn reset_camera(
    mut commands: Commands,
    launcher_q: Query<&Transform, Added<Launcher>>,
    mut look_q: Query<&mut LookTransform, With<MainCamera>>,
) {
    for launcher_trans in launcher_q.iter() {
        if let Ok(mut look) = look_q.get_single_mut() {
            let delta = Vec2::new(0., PI / 16.);
            let mut angles = LookAngles::from_vector(look.look_direction().unwrap());
            angles.set_pitch(delta.y);
            angles.set_yaw(delta.x);

            let offset = 6.;

            // Third-person.
            look.eye = launcher_trans.translation + offset * angles.unit_vector();

            look.target = launcher_trans.translation;
        }
    }
}



fn ball_follow_camera(
    ball_q: Query<&Transform, With<GolfBall>>,
    mut camera_q: Query<&mut LookTransform, With<MainCamera>>,
) {
    if let Ok(ball_trans) = ball_q.get_single() {
        if let Ok(mut look) = camera_q.get_single_mut() {
            
            let delta = Vec2::new(0., PI / 16.);
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