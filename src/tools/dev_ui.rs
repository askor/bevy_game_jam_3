use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{egui::{self, Visuals}, bevy_egui::EguiContext};

use crate::game::{level::level_manager::SaveLevelEvent, gameplay_elements::{LaunchEvent, launcher::LaunchVelocity}};

pub struct DevUiPlugin;

impl Plugin for DevUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup_ui);
    }
}

fn setup_ui(
    world: &mut World,
) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();
    
    egui::Window::new("Settings").show(egui_context.get_mut(), |ui| {
        ui.ctx().set_visuals(Visuals::light());
        ui.label("Level");
        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                world.send_event::<SaveLevelEvent>(SaveLevelEvent);
            }
            if ui.button("Load").clicked() {
                info!("Button 2 clicked!");
            }
        });
        ui.label("Some label");
        if ui.button("Launch").clicked() {
            world.send_event::<LaunchEvent>(LaunchEvent);
        }

        let mut float = 1.0;
        ui.add(egui::Slider::new(&mut world.resource_mut::<LaunchVelocity>().0, 10.0..=100.0));
        
        let mut value = true;
        ui.horizontal(|ui| {
            ui.label("Selectable:");
            ui.selectable_value(&mut value, true, "On");
            ui.selectable_value(&mut value, false, "Off");
        });
    });
}