use bevy::prelude::*;

pub const GRID_WIDTH: usize = 20;
pub const GRID_HEIGHT: usize = 20;

pub const MOVEMENT_INTERVAL: std::time::Duration = std::time::Duration::from_millis(100);
pub const MIN_MOVEMENT_INTERVAL: std::time::Duration = std::time::Duration::from_millis(50);
pub const SPEED_STEP_FOODS: i32 = 5;
pub const SPEED_STEP_MILLIS: u64 = 8;

pub const TILE_SIZE: f32 = 32.0;
pub const SEGMENT_SPACING: f32 = TILE_SIZE * 0.92;
pub const SELF_COLLISION_THRESHOLD: f32 = TILE_SIZE * 0.4;

pub const COLOR_BG: Color = Color::srgb(0.06, 0.08, 0.06);
pub const COLOR_GRID_LINE: Color = Color::srgb(0.1, 0.13, 0.10);
pub const COLOR_SNAKE_BODY: Color = Color::srgb(0.15, 0.85, 0.15);
pub const COLOR_SNAKE_HEAD: Color = Color::srgb(0.45, 1.0, 0.20);
pub const COLOR_FOOD: Color = Color::srgb(0.95, 0.10, 0.10);
pub const COLOR_GAME_OVER: Color = Color::srgba(0.0, 0.0, 0.0, 0.45);

pub const WORLD_LEFT: f32 = 0.0;
pub const WORLD_RIGHT: f32 = (GRID_WIDTH as f32) * TILE_SIZE;
pub const WORLD_BOTTOM: f32 = 0.0;
pub const WORLD_TOP: f32 = (GRID_HEIGHT as f32) * TILE_SIZE;

pub(crate) const HIGH_SCORE_KEY: &str = "snake-highest";
