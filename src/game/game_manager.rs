use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{AppState, loading::FontAssets};

use super::gameplay_elements::Goal;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Standby,
    InProgress,
    Complete,
}
pub struct LevelCompletEvent;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_event::<LevelCompletEvent>()
            .add_system(start_game.in_schedule(OnEnter(AppState::Playing)))
            .add_system(level_complete
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(hud_level_complete.run_if(on_event::<LevelCompletEvent>()))
            // .add_system(hud_level_complete.in_schedule(OnEnter(GameState::Complete)))
            .add_system(hud_status_reset.in_schedule(OnExit(GameState::Complete)))
            .add_system(setup_hud.in_schedule(OnEnter(AppState::Playing)))
            .add_system(cleanup_hud.in_schedule(OnExit(AppState::Playing)))
            ;
    }
}

fn start_game(
    mut state: ResMut<NextState<GameState>>,
) {
    state.set(GameState::InProgress);
}

fn level_complete(
    mut collisions: EventReader<CollisionEvent>,
    q_entity: Query<Entity, With<Goal>>,
    mut state: ResMut<NextState<GameState>>,
    mut events: EventWriter<LevelCompletEvent>,
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
                    events.send(LevelCompletEvent);
                    state.set(GameState::Complete);
                }
            },
        }
    }
}

fn hud_level_complete(
    mut text_query: Query<&mut Text>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = "Level Complete".to_string();
    }
}

fn hud_status_reset(
    mut text_query: Query<&mut Text>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = "".to_string();
    }
}

#[derive(Component)]
struct HudGameStatus;

fn setup_hud(
    mut commands: Commands,
    fonts: Res<FontAssets>,
) {
    info!("HUD");
    let font = fonts.fira_sans.clone();
    let text_style = TextStyle {
        font,
        font_size: 80.0,
        color: Color::rgb(200./256., 200./256., 200./256.),
    };
    
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    })
    .insert(Name::new("HUD"))
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "",
                text_style.clone(),
            ).with_style(Style {
                size: Size::new(Val::Undefined, Val::Px(100.)),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Px(10.0),
                    top: Val::Px(6.0),
                    bottom: Val::Auto,
                },
                ..default()
            }),
            HudGameStatus,
        ));
    });
}

fn cleanup_hud(
    mut commands: Commands,
    hud_query: Query<Entity, With<Node>>,
) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}