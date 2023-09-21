mod dev_ui;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use self::dev_ui::DevUiPlugin;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(DevUiPlugin);
    }
}