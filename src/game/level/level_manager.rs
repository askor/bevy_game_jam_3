use std::{fs::File, io::Write};
use bevy::{prelude::*, tasks::IoTaskPool};
use crate::game::{game_manager::GameState, gameplay_elements::{goal::Goal, launcher::Launcher, wall}};
use crate::game::gameplay_elements::ball::GolfBall;

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SaveLevelEvent>()
            .add_event::<LoadLevelEvent>()
            .add_system(load_level_system)
            .add_system(save_scene_system.run_if(on_event::<SaveLevelEvent>()))
            // .add_system(load_scene_system.in_schedule(OnEnter(GameState::InProgress)))
            .add_system(clean_up_level.in_schedule(OnExit(GameState::Complete)))
            ;
    }
}

pub struct SaveLevelEvent{
    pub name: String,
}
pub struct LoadLevelEvent {
    pub level: usize,
}

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

// load level
fn load_level_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<LoadLevelEvent>,
) {
    for event in events.iter() {
        info!("Loading level!");
        commands.spawn((
            Level,
            DynamicSceneBundle {
                scene: asset_server.load(format!("levels/level_{}.scn.ron", event.level)),
                visibility: Visibility::Visible,
                ..default()
            },
            Name::new("Level"),
        ));
    }
}

fn save_scene_system(
    world: &mut World,
) {
    info!("Saving!");

    let events = world.resource::<Events<SaveLevelEvent>>();
    let mut event_reader = events.get_reader();
    // let filename = event_reader.iter(SaveLevelEvent).last();

    let name = event_reader.iter(events).last().unwrap().name.clone();

    info!("Name {}", name);

    let custom_type_registry = AppTypeRegistry::default();

    // let mut custom_registry = scene_world.resource_mut::<AppTypeRegistry>();
    custom_type_registry.write().register::<GolfBall>();
    custom_type_registry.write().register::<Launcher>();
    custom_type_registry.write().register::<Goal>();
    custom_type_registry.write().register::<wall::Box>();
    custom_type_registry.write().register::<wall::PlainWall>();
    custom_type_registry.write().register::<wall::BounceWall>();
    custom_type_registry.write().register::<wall::LowGravWall>();
    
    // custom_type_registry.write().register::<GlobalTransform>();
    // custom_type_registry.write().register::<Affine3A>();
    // custom_type_registry.write().register::<Mat3A>();
    // custom_type_registry.write().register::<Vec3A>();

    custom_type_registry.write().register::<Transform>();
    custom_type_registry.write().register::<Vec3>();
    custom_type_registry.write().register::<Quat>();

    info!("After builder");

    let mut query = world.query_filtered::<(Entity, &Children), With<Level>>();
    let (_level_entity, children) = match query.get_single(world) {
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
            File::create(format!("assets/levels/{name}.scn.ron"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}
