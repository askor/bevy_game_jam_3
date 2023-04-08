use std::{fs::File, io::Write};
use bevy::{prelude::*, tasks::IoTaskPool, math::{Affine3A, Mat3A, Vec3A}};
use crate::{game::{game_manager::GameState, gameplay_elements::{Goal, self, launcher::Launcher}}, AppState};
use crate::game::gameplay_elements::ball::GolfBall;

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SaveLevelEvent>()
            .add_system(save_scene_system.run_if(on_event::<SaveLevelEvent>()))
            .add_system(load_scene_system.in_schedule(OnEnter(GameState::InProgress)))
            .add_system(clean_up_level.in_schedule(OnExit(GameState::Complete)))
            ;
    }
}

pub struct SaveLevelEvent;

// Current level

// clean up level
fn clean_up_level(
    mut commands: Commands,
    query: Query<Entity, With<Level>>,
) {
    for level in query.iter() {
        commands.entity(level).despawn_recursive();
    }
}

// save level (scene)
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Level;

const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";

// load level
fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Level,
        DynamicSceneBundle {
            scene: asset_server.load(NEW_SCENE_FILE_PATH),
            visibility: Visibility::Visible,
            ..default()
        },
        Name::new("Level"),
    ));
}

fn save_scene_system(
    // mut events: EventReader<SaveLevelEvent>,
    world: &mut World,
) {
    info!("Saving!");

    let custom_type_registry = AppTypeRegistry::default();

    // let mut custom_registry = scene_world.resource_mut::<AppTypeRegistry>();
    custom_type_registry.write().register::<GolfBall>();
    custom_type_registry.write().register::<Launcher>();
    custom_type_registry.write().register::<Goal>();
    custom_type_registry.write().register::<gameplay_elements::Box>();
    
    // custom_type_registry.write().register::<GlobalTransform>();
    // custom_type_registry.write().register::<Affine3A>();
    // custom_type_registry.write().register::<Mat3A>();
    // custom_type_registry.write().register::<Vec3A>();

    custom_type_registry.write().register::<Transform>();
    custom_type_registry.write().register::<Vec3>();
    custom_type_registry.write().register::<Quat>();

    let type_registry = world.resource::<AppTypeRegistry>().clone();

    info!("After builder");

    let mut query = world.query_filtered::<(Entity, &Children), With<Level>>();
    let (level_entity, children) = match query.get_single(world) {
        Ok(v) => v,
        Err(_) => return,
    };

    info!("After query");

    // let mut builder = DynamicSceneBuilder::from_world(world);
    let mut builder = DynamicSceneBuilder::from_world_with_type_registry(world, custom_type_registry.clone());

    for child in children {
        builder.extract_entity(*child);
        info!("child: {:?}", child);
            // builder.extract_entity(*child);
    }
    // builder.extract_entities(children.to_owned().into_iter());
        // world.entity_mut(level_entity).despawn_recursive();

    let scene = builder.build();
    info!("Scene built");

    // Scenes can be serialized like this:
    // let serialized_scene = scene.serialize_ron(&type_registry).unwrap();
    let serialized_scene = scene.serialize_ron(&custom_type_registry).unwrap();

    // Showing the scene in the console
    info!("{}", serialized_scene);

    // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
    // as they are blocking
    // This can't work in WASM as there is no filesystem access
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}
