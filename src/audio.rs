// use crate::GameState;
use bevy::prelude::*;

use crate::{loading::AudioAssets, AppState};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_system(play_music.in_schedule(OnEnter(AppState::Playing)))
            // .add_system(play_music
            //     .in_set(OnUpdate(GameState::InProgress))
            //     .in_set(OnUpdate(AppState::Playing))
            // )
            ;
    }
}

// fn play_menu_music(
//     assets: Res<AudioAssets>,
//     audio: Res<Audio>,
// ) {
//     let music = assets.main_theme.clone();
//     audio.play(music);
// }

fn play_music(
    assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    let music = assets.main_theme.clone();
    audio.play(music);
}