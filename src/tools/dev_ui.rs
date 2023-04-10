use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{egui::{self, Visuals}, bevy_egui::EguiContext};

use crate::game::{level::{level_manager::SaveLevelEvent, Level}, gameplay_elements::{LaunchEvent, launcher::LaunchVelocity, wall::{LowGravWall, BounceWall, PlainWall, Box}}, GameState};

pub struct DevUiPlugin;

impl Plugin for DevUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup_ui)
            ;
    }
}

struct UiState {
    name_input: String,
}

impl Default for UiState {
    fn default() -> Self {
        Self { name_input: "level_1".to_string() }
    }
}

fn setup_ui(
    world: &mut World,
    mut state: Local<UiState>,
    mut box_dims: Local<Box>,
) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();
    
    egui::Window::new("Settings").show(egui_context.get_mut(), |ui| {
        ui.ctx().set_visuals(Visuals::light());
        ui.label("Level");
        ui.text_edit_singleline(&mut state.name_input);
        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                world.send_event::<SaveLevelEvent>(SaveLevelEvent {name: state.name_input.to_string() });
            }
            if ui.button("Load").clicked() {
                info!("Button 2 clicked!");
            }
        });
        ui.label("Level Controls");
        if ui.button("Standby").clicked() {
            world.resource_mut::<NextState<GameState>>().set(GameState::Standby);
        }
        if ui.button("InProgress").clicked() {
            world.resource_mut::<NextState<GameState>>().set(GameState::InProgress);
        }
        if ui.button("Complete game").clicked() {
            world.resource_mut::<NextState<GameState>>().set(GameState::Complete);
        }
        ui.label("Level creator");
        ui.horizontal(|ui| {
            ui.label("Box:");
            ui.add(egui::DragValue::new(&mut box_dims.x));
            ui.add(egui::DragValue::new(&mut box_dims.y));
            ui.add(egui::DragValue::new(&mut box_dims.z));
        });
        ui.horizontal(|ui| {
            if ui.button("Plain ground").clicked() {
                let level_entity = world.query_filtered::<Entity, With<Level>>().single(world);
                let id = world.spawn(PlainWall).insert((Box{ x: box_dims.x, y: box_dims.y, z: box_dims.z }, SpatialBundle::default())).id();
                world.entity_mut(level_entity).add_child(id);
            }
            if ui.button("Bounce Wall").clicked() {
                info!("Button 2 clicked!");
                let level_entity = world.query_filtered::<Entity, With<Level>>().single(world);
                let id = world.spawn(BounceWall).insert((Box{ x: box_dims.x, y: box_dims.y, z: box_dims.z }, SpatialBundle::default())).id();
                world.entity_mut(level_entity).add_child(id);
            }
            if ui.button("Gravity Wall").clicked() {
                info!("Button 2 clicked!");
                let level_entity = world.query_filtered::<Entity, With<Level>>().single(world);
                let id = world.spawn(LowGravWall).insert((Box{ x: box_dims.x, y: box_dims.y, z: box_dims.z }, SpatialBundle::default())).id();
                world.entity_mut(level_entity).add_child(id);
            }
        });

        ui.label("Launch velocity:");
        ui.add(egui::Slider::new(&mut world.resource_mut::<LaunchVelocity>().0, 10.0..=100.0));
        
        // let mut value = true;
        // ui.horizontal(|ui| {
        //     ui.selectable_value(&mut value, true, "On");
        //     ui.selectable_value(&mut value, false, "Off");
        // });
    });
}