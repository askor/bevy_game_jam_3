use bevy::prelude::*;

pub struct GameplayElementsPlugin;

impl Plugin for GameplayElementsPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component)]
pub(crate) struct Goal;

#[derive(Component)]
pub(crate) struct GolfBall;