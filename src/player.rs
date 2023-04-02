use crate::actions::Actions;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

const SPEED: f32 = 10.;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_player.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_player(
    mut commands: Commands,
) {
    commands
        .spawn((
            Camera3dBundle::default(),
            Player,
        ));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * SPEED * time.delta_seconds(),
        actions.player_movement.unwrap().y * SPEED * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
