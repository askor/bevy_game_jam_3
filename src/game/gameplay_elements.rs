use bevy::prelude::*;

pub struct GameplayElementsPlugin;

impl Plugin for GameplayElementsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Goal>()
            .register_type::<GolfBall>();
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct Goal;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct GolfBall;