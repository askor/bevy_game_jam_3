use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{AppState, loading::FontAssets, menu::ButtonColors};

use super::{gameplay_elements::Goal, level::level_manager::LoadLevelEvent};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Standby,
    InProgress,
    Complete,
}
pub struct LevelCompletEvent;
pub struct GameCompleteEvent;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_event::<LevelCompletEvent>()
            .add_event::<GameCompleteEvent>()
            .init_resource::<CurrentLevel>()
            .add_system(auto_start_game.in_schedule(OnEnter(AppState::Playing)))
            .add_system(auto_end_level
                .in_set(OnUpdate(GameState::Complete))
            )
            .add_system(auto_load_next_level
                .in_set(OnUpdate(GameState::Standby))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(level_complete
                .in_set(OnUpdate(GameState::InProgress))
                .in_set(OnUpdate(AppState::Playing))
            )
            .add_system(hud_game_complete.run_if(on_event::<GameCompleteEvent>()))
            .add_system(hud_level_complete.run_if(on_event::<LevelCompletEvent>()))
            // .add_system(hud_level_complete.in_schedule(OnEnter(GameState::Complete)))
            .add_system(hud_status_reset.in_schedule(OnExit(GameState::Complete)))
            .add_system(setup_hud.in_schedule(OnEnter(AppState::Playing)))
            .add_system(cleanup_hud.in_schedule(OnExit(AppState::Playing)))
            ;
    }
}

const LEVEL_COUNT: usize = 2;

#[derive(Resource)]
struct CurrentLevel {
    index: usize,
    shots: usize,
}

impl Default for CurrentLevel {
    fn default() -> Self {
        Self { index: 1, shots: 0 }
    }
}

#[derive(Resource)]
struct NewLevelTimer {
    timer: Timer,
}

impl Default for NewLevelTimer {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(2.0, TimerMode::Once) }
    }
}

fn auto_start_game(
    mut state: ResMut<NextState<GameState>>,
    mut events: EventWriter<LoadLevelEvent>,
) {
    state.set(GameState::InProgress);

    events.send(LoadLevelEvent { level: 1 });
}

fn auto_load_next_level (
    mut state: ResMut<NextState<GameState>>,
    mut events: EventWriter<LoadLevelEvent>,
    mut local: Local<NewLevelTimer>,
    time: Res<Time>,
) {
    if local.timer.tick(time.delta()).finished() {
        state.set(GameState::InProgress);
        events.send(LoadLevelEvent { level: 2 });
        local.timer.reset();
    }
}

fn auto_end_level (
    mut state: ResMut<NextState<GameState>>,
    mut local: Local<NewLevelTimer>,
    time: Res<Time>,
    mut current_level: ResMut<CurrentLevel>,
    mut game_complete: EventWriter<GameCompleteEvent>,
) {
    if local.timer.tick(time.delta()).just_finished() {
        current_level.index +=1;
        if current_level.index > LEVEL_COUNT {
            info!("GAME COMPLETE");
            game_complete.send(GameCompleteEvent);
        } else {
            local.timer.reset();
            state.set(GameState::Standby);
        }
    }
}

fn level_complete(
    mut collisions: EventReader<CollisionEvent>,
    q_entity: Query<Entity, With<Goal>>,
    mut state: ResMut<NextState<GameState>>,
    mut events: EventWriter<LevelCompletEvent>,
) {
    for collision in collisions.iter() {
        
        match collision {
            CollisionEvent::Started(a, b, _) => {
                if q_entity.get(*a).is_ok() || q_entity.get(*b).is_ok() {
                    info!("Game over!");
                    events.send(LevelCompletEvent);
                    state.set(GameState::Complete);
                }
            },
            CollisionEvent::Stopped(_, _, _) => (),
        }
    }
}

fn hud_game_complete(
    mut text_query: Query<&mut Text>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = "ALL LEVELS COMPLETE".to_string();
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
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    info!("HUD");
    let font = fonts.fira_sans.clone();
    let text_style = TextStyle {
        font,
        font_size: 80.0,
        color: Color::rgb(200./256., 200./256., 200./256.),
    };

    // commands.spawn(NodeBundle {
    //     style: Style {
    //         justify_content: JustifyContent::SpaceAround,
    //         align_items: AlignItems::Center,
    //         ..default()
    //     },
    //     ..default()
    // })
    // .with_children(|cmd| {

        
    // });
    // commands.spawn(ButtonBundle {
    //     style: Style {
    //         size: Size::new(Val::Px(120.0), Val::Px(50.0)),
    //         margin: UiRect::all(Val::Auto),
    //         justify_content: JustifyContent::Center,
    //         align_items: AlignItems::Center,
    //         ..Default::default()
    //     },
    //     background_color: button_colors.normal.into(),
    //     ..Default::default()
    // })
    // .with_children(|parent| {
    //     parent.spawn(TextBundle::from_section(
    //         "Play",
    //         TextStyle {
    //             font: font_assets.fira_sans.clone(),
    //             font_size: 40.0,
    //             color: Color::rgb(0.9, 0.9, 0.9),
    //         },
    //     ));
    // });

    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
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
                // margin: UiRect {
                //     left: Val::Auto,
                //     right: Val::Px(10.0),
                //     top: Val::Px(6.0),
                //     bottom: Val::Auto,
                // },
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