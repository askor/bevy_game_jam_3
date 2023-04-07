use crate::AppState;
use bevy::{prelude::*, audio::Source};
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::Menu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(AppState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/shots/Shot1.ogg")]
    pub launch1: Handle<AudioSource>,
    #[asset(path = "audio/shots/Shot2.ogg")]
    pub launch2: Handle<AudioSource>,
    #[asset(path = "audio/shots/Shot3.ogg")]
    pub launch3: Handle<AudioSource>,
    #[asset(path = "audio/shots/Shot4.ogg")]
    pub launch4: Handle<AudioSource>,
    
    // Music
    #[asset(path = "audio/music/MainThemeLoop.ogg")]
    pub main_theme: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}
