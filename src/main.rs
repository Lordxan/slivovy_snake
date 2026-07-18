pub mod core;
mod plugins;

use bevy::prelude::*;
use plugins::{
    CollisionPlugin, FoodPlugin, GameCorePlugin, GameEventsPlugin, GameFlowPlugin, InputPlugin,
    ParticlePlugin, RenderingPlugin, SnakePlugin, UIPlugin,
};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
