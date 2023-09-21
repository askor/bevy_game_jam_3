mod actions;
mod audio;
mod loading;
mod menu;
mod player;
mod environment;
mod game;
mod tools;
mod camera;

use actions::ActionsPlugin;
use audio::InternalAudioPlugin;
use camera::InternalCameraPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use environment::EnvironmentPlugin;
use game::GamePlugin;
// use player::PlayerPlugin;
// use tools::ToolsPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    #[default]
    Loading,
    Playing,
    Menu,
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalCameraPlugin)
            .add_plugin(EnvironmentPlugin)
            .add_plugin(GamePlugin)
            
            // .add_plugin(PlayerPlugin)
            // .add_plugin(ToolsPlugin)
            ;

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}
