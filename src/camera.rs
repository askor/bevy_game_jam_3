use std::f32::consts::PI;

use bevy::{prelude::*, core_pipeline::bloom::BloomSettings};
use smooth_bevy_cameras::{LookTransformBundle, LookTransform, Smoother, LookTransformPlugin, LookAngles};

use crate::{AppState, game::{gameplay_elements::{LaunchEvent, launcher::Launcher}, GameState}, player::{FlyCam, NoCameraPlayerPlugin}};
use crate::game::gameplay_elements::ball::GolfBall;

pub struct InternalCameraPlugin;

impl Plugin for InternalCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LookTransformPlugin)
            .add_plugin(NoCameraPlayerPlugin)
            .insert_resource(FreeCam(false))
            .add_system(setup.in_schedule(OnEnter(AppState::Playing)))
            .add_system(ball_follow_camera
                .in_set(OnUpdate(AppState::Playing))
                .run_if(not(free_cam))
            )
            // .add_system(ball_follow_camera
            //     .in_set(OnUpdate(AppState::Playing))
            //     .run_if(not(free_cam))
            // )
            .add_system(reset_camera
                .in_set(OnUpdate(AppState::Playing))
                .in_set(OnUpdate(GameState::InProgress))
                .run_if(not(free_cam))
            )
            .add_system(toggle_freecam_stuff)
            ;
    }
}

#[derive(Resource)]
pub struct FreeCam(bool);

pub fn free_cam(res: Res<FreeCam>) -> bool { res.0 }

fn toggle_freecam_stuff (
    mut commands: Commands,
    query: Query<Entity, With<MainCamera>>,
    mut free_cam: ResMut<FreeCam>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::T) {
        if free_cam.0 {
            let cam = query.single();
            commands.entity(cam).remove::<FlyCam>();
            commands.entity(cam).insert(LookTransform{ eye: Vec3::ZERO, target: Vec3::ZERO, up: Vec3::Y});
            free_cam.0 = false;
        } else {
            let cam = query.single();
            commands.entity(cam).insert(FlyCam);
            commands.entity(cam).remove::<LookTransform>();
            free_cam.0 = true;
        }
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
    launcher_q: Query<&Transform, With<Launcher>>,
    mut look_q: Query<&mut LookTransform, With<MainCamera>>,
    ball_q: Query<&GolfBall>,
) {
    if let Ok(_) = ball_q.get_single() {
        return;
    }
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

            let offset = look.radius().clamp(10., 40.);

            // Third-person.
            look.eye = look.target + offset * angles.unit_vector();

            look.target = ball_trans.translation;
        }
    }
}