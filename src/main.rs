pub mod core;
mod plugins;

use bevy::{
    prelude::*,
    window::{Window, WindowPlugin},
};
use plugins::{
    CollisionPlugin, FoodPlugin, GameCorePlugin, GameEventsPlugin, GameFlowPlugin, InputPlugin,
    ParticlePlugin, RenderingPlugin, SnakePlugin, UIPlugin,
};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#game".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            CollisionPlugin,
            FoodPlugin,
            GameCorePlugin,
            GameEventsPlugin,
            ParticlePlugin,
            SnakePlugin,
            InputPlugin,
            RenderingPlugin,
            GameFlowPlugin,
            UIPlugin,
        ))
        .run();
}
