use bevy::prelude::*;

use crate::{
    core::{
        global::{HighScore, Score},
        utils::get_high_score,
    },
    plugins::{
        food::FoodEntity,
        snake::PendingDirection,
        ui::{
            cleanup_game_over_screen, cleanup_start_screen, setup_game_over, setup_start_screen,
            spawn_hud, HudGroup,
        },
    },
};

pub fn setup_gameplay(
    mut commands: Commands,
    mut movement_timer: ResMut<crate::core::global::MovementTimer>,
    mut score: ResMut<Score>,
    mut high_score: ResMut<HighScore>,
    mut pending: ResMut<PendingDirection>,
) {
    movement_timer.reset();
    score.0 = 0;
    high_score.0 = get_high_score();
    pending.0 = None;
    spawn_hud(&mut commands);
}

pub fn cleanup_gameplay(
    mut commands: Commands,
    food: Query<Entity, With<FoodEntity>>,
    hud_group: Query<Entity, With<HudGroup>>,
) {
    for entity in food.iter() {
        commands.entity(entity).despawn();
    }
    for entity in hud_group.iter() {
        commands.entity(entity).despawn();
    }
}

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<crate::core::global::GameState>()
            .add_systems(
                OnEnter(crate::core::global::GameState::Playing),
                setup_gameplay,
            )
            .add_systems(
                OnExit(crate::core::global::GameState::Playing),
                cleanup_gameplay,
            )
            .add_systems(
                OnEnter(crate::core::global::GameState::StartScreen),
                setup_start_screen,
            )
            .add_systems(
                OnExit(crate::core::global::GameState::StartScreen),
                cleanup_start_screen,
            )
            .add_systems(
                OnEnter(crate::core::global::GameState::GameOver),
                setup_game_over,
            )
            .add_systems(
                OnExit(crate::core::global::GameState::GameOver),
                cleanup_game_over_screen,
            );
    }
}
