// use crate::GameState;
use bevy::prelude::*;

use crate::loading::AudioAssets;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(play_music);
    }
}

fn play_music(
    asset_server: Res<AssetServer>,
    // assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    let music = asset_server.load("audio/flog2.mp3");
    audio.play(music);
}