use bevy::prelude::*;

use crate::core::constants::{
    MIN_MOVEMENT_INTERVAL, MOVEMENT_INTERVAL, SPEED_STEP_FOODS, SPEED_STEP_MILLIS,
};
use crate::core::global::{FoodsEaten, GameSets, HighScore, MovementTimer, Score};
use crate::core::utils::set_high_score;
use crate::plugins::particle::{ParticleBurstEvents, ParticleFlashEvents};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameEventsSets {
    Events,
}

pub fn emit_eat_particles(
    mut burst_events: ResMut<ParticleBurstEvents>,
    mut flash_events: ResMut<ParticleFlashEvents>,
    mut eat_events: ResMut<crate::plugins::food::EatEvents>,
) {
    for pos in &eat_events.0 {
        burst_events.0.push(*pos);
        flash_events.0.push(*pos);
    }

    eat_events.0.clear();
}

pub fn update_score_and_speed(
    mut movement_timer: ResMut<MovementTimer>,
    foods_eaten: Res<FoodsEaten>,
) {
    let foods = foods_eaten.0;
    if foods <= 0 {
        return;
    }
    let steps = foods / SPEED_STEP_FOODS;
    let current_ms = (MOVEMENT_INTERVAL.as_millis() as u64 - (steps as u64) * SPEED_STEP_MILLIS)
        .max(MIN_MOVEMENT_INTERVAL.as_millis() as u64);
    (**movement_timer).set_duration(std::time::Duration::from_millis(current_ms));
}

pub fn check_game_over_state(
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
    mut foods_eaten: ResMut<FoodsEaten>,
) {
    if score.0 > high_score.0 {
        high_score.0 = score.0;
        set_high_score(score.0);
    }
    foods_eaten.0 = 0;
}

pub struct GameEventsPlugin;

impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                emit_eat_particles.in_set(GameEventsSets::Events),
                update_score_and_speed
                    .after(crate::plugins::snake::SnakeSets::Movement)
                    .in_set(GameEventsSets::Events),
                check_game_over_state
                    .run_if(in_state(crate::core::global::GameState::GameOver))
                    .in_set(GameEventsSets::Events),
            )
                .in_set(GameSets::GameEvents),
        );
    }
}
