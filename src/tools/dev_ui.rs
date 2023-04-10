use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{egui::{self, Visuals}, bevy_egui::EguiContext};

use crate::game::{level::level_manager::SaveLevelEvent, gameplay_elements::{LaunchEvent, launcher::LaunchVelocity}, GameState};

pub struct DevUiPlugin;

impl Plugin for DevUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup_ui);
    }
}

#[derive(Default)]
struct UiState {
    name_input: String,
}

fn setup_ui(
    world: &mut World,
    mut state: Local<UiState>,
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

        ui.label("Launch velocity:");
        ui.add(egui::Slider::new(&mut world.resource_mut::<LaunchVelocity>().0, 10.0..=100.0));
        
        // let mut value = true;
        // ui.horizontal(|ui| {
        //     ui.selectable_value(&mut value, true, "On");
        //     ui.selectable_value(&mut value, false, "Off");
        // });
    });
}