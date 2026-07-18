use bevy::prelude::*;

use crate::core::global::{GameSets, GameState, MovementTimer, Score};
use crate::plugins::snake::{Direction, PendingDirection};

pub fn read_input(
    mut pending_direction: ResMut<PendingDirection>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let new_dir = || -> Option<Direction> {
        if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
            return Some(Direction::Up);
        } else if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS)
        {
            return Some(Direction::Down);
        } else if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA)
        {
            return Some(Direction::Left);
        } else if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD)
        {
            return Some(Direction::Right);
        };

        return None;
    };

    if let (Some(dir), None) = (new_dir(), pending_direction.0) {
        pending_direction.0 = Some(dir);
    }
}

pub fn start_input(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut movement_timer: ResMut<MovementTimer>,
    mut pending: ResMut<PendingDirection>,
) {
    if !keyboard.get_just_pressed().any(|k| {
        matches!(
            k,
            KeyCode::ArrowUp
                | KeyCode::ArrowDown
                | KeyCode::ArrowLeft
                | KeyCode::ArrowRight
                | KeyCode::KeyW
                | KeyCode::KeyA
                | KeyCode::KeyS
                | KeyCode::KeyD
                | KeyCode::Space
                | KeyCode::Enter
        )
    }) {
        return;
    }

    score.0 = 0;
    movement_timer.reset();
    pending.0 = None;

    next_state.set(GameState::Playing);
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, read_input).add_systems(
            Update,
            (
                start_input.run_if(in_state(GameState::GameOver)),
                start_input.run_if(in_state(GameState::StartScreen)),
            )
                .in_set(GameSets::Input),
        );
    }
}
