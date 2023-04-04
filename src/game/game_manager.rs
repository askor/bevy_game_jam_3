use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::AppState;

use super::gameplay_elements::Goal;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub(crate) enum GameState {
    #[default]
    Standby,
    InProgress,
    Complete,
}

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_system(load_level.in_schedule(OnEnter(GameState::InProgress)))
            .add_system(level_complete
                // .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            ;
    }
}

fn load_level() {
    // TODO
    info!("TODO: Implement level loading");
}

fn level_complete(
    mut collisions: EventReader<CollisionEvent>,
    q_entity: Query<Entity, With<Goal>>,
    mut state: ResMut<NextState<GameState>>,
) {
    for collision in collisions.iter() {
        
        match collision {
            CollisionEvent::Stopped(_, _, _) => return,
            CollisionEvent::Started(a, b, _) => {
                if q_entity.get(*a).is_ok() {
                    info!("Game over!");
                }
                if q_entity.get(*b).is_ok() {
                    info!("Game over!");
                    state.set(GameState::Complete);
                }
            },
        }
    }
}