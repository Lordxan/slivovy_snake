use bevy::prelude::*;

use crate::core::constants::MOVEMENT_INTERVAL;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    StartScreen,
    Playing,
    GameOver,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSets {
    Input,
    Visuals,
    GameEvents,
    GameLogic,
}

#[derive(Resource)]
pub struct HighScore(pub i32);

impl Default for HighScore {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct Score(pub i32);

impl Default for Score {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct FoodsEaten(pub i32);

impl Default for FoodsEaten {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct MovementTimer(Timer);

impl Default for MovementTimer {
    fn default() -> Self {
        Self(Timer::new(MOVEMENT_INTERVAL, TimerMode::Repeating))
    }
}
