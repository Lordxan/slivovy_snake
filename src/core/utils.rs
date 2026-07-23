use bevy::prelude::*;
use rand::Rng;

use super::constants::{
    HIGH_SCORE_KEY, MIN_MOVEMENT_INTERVAL, MOVEMENT_INTERVAL, SEGMENT_SPACING, SPEED_STEP_FOODS,
    SPEED_STEP_MILLIS, TILE_SIZE, WORLD_RIGHT, WORLD_TOP,
};

pub fn get_high_score() -> i32 {
    match get_local_storage_value(HIGH_SCORE_KEY) {
        Some(v) => v.parse::<i32>().unwrap_or(0),
        None => 0,
    }
}

pub fn set_high_score(score: i32) {
    let current = get_high_score();
    if score <= current {
        return;
    }
    set_local_storage_value(HIGH_SCORE_KEY, &score.to_string());
}

#[cfg(target_arch = "wasm32")]
pub fn get_local_storage_value(key: &str) -> Option<String> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .map(|storage| storage.get(key).ok().flatten())
        .unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
pub fn set_local_storage_value(key: &str, value: &str) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.set(key, value);
    }
}

pub fn random_food_position(snake_segments: &[Vec2]) -> Vec2 {
    let mut rng = rand::rng();
    let margin = TILE_SIZE * 3.;
    let spacing = SEGMENT_SPACING * 0.8;
    loop {
        let pos = Vec2::new(
            rng.random_range(margin..WORLD_RIGHT - margin),
            rng.random_range(margin..WORLD_TOP - margin),
        );
        let collision = snake_segments.iter().any(|seg| seg.distance(pos) < spacing);
        if !collision {
            return pos;
        }
    }
}

pub fn calculate_speed_mult(foods_eaten: i32) -> i32 {
    if foods_eaten <= 0 {
        return 1;
    }
    let steps = foods_eaten / SPEED_STEP_FOODS;
    let current_ms = (MOVEMENT_INTERVAL.as_millis() as u64 - (steps as u64) * SPEED_STEP_MILLIS)
        .max(MIN_MOVEMENT_INTERVAL.as_millis() as u64);
    let base_ms = MOVEMENT_INTERVAL.as_millis() as f32;
    let mult = (base_ms / current_ms as f32).round() as i32;
    if mult == 0 {
        1
    } else {
        mult
    }
}
