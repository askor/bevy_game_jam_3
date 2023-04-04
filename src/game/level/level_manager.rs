use std::{fs::File, io::Write};

use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{game::game_manager::GameState, AppState};

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<TestLevelComponent>()
            .register_type::<TestSceneComponent>()
            .add_system(create_test_level.in_schedule(OnEnter(AppState::Playing)))
            .add_system(save_scene_system.in_schedule(OnEnter(GameState::Complete)))
            ;
    }
}

// Current level

// load next level

// clean up level

// save level (scene)
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct TestLevelComponent;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct TestSceneComponent;

fn create_test_level(
    mut commands: Commands
) {
    info!("Spawn!");

    commands
        .spawn(TestLevelComponent)
        .with_children(|parent| {
            parent.spawn(TestSceneComponent);
        });
}

const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";

fn save_scene_system(
    world: &mut World,
) {
    info!("Saving!");

    // let mut scene_world = World::new();
    
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    
    info!("After builder");
    
    let mut query = world.query_filtered::<(Entity, &Children), With<TestLevelComponent>>();
    let (level_entity, children) = match query.get_single(world) {
        Ok(v) => v,
        Err(_) => return,
    };
    
    info!("After query");
    
    let mut builder = DynamicSceneBuilder::from_world(world);
    for child in children {
        // builder.extract_entity(*child);
            info!("child: {:?}", child);
            builder.extract_entity(level_entity);
    }
    // builder.extract_entities(children.to_owned().into_iter());
        // world.entity_mut(level_entity).despawn_recursive();

    let scene = builder.build();
    info!("Scene built");

    // Scenes can be serialized like this:
    let serialized_scene = scene.serialize_ron(&type_registry).unwrap();

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
