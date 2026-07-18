use bevy::prelude::*;

use crate::plugins::food::FoodEntity;

pub fn animate_food_pulse(
    mut foods: Query<(&mut Transform, &mut Sprite), With<FoodEntity>>,
    time: Res<Time>,
) {
    let t = time.elapsed_secs() * 3.0;
    let scale = 0.85 + (t.cos() * 0.08);
    if foods.iter().len() != 1 {
        return;
    }
    let (mut transform, mut sprite) = foods.single_mut().unwrap();
    transform.scale = Vec3::splat(scale);
    let glow_factor = t.sin().max(0.0) * 0.15;
    let mut color = sprite.color;
    if let Color::Srgba(c) = color {
        color = Color::srgba(
            (c.red + glow_factor).min(1.0),
            (c.green + glow_factor * 0.5).min(1.0),
            (c.blue + glow_factor * 0.3).min(1.0),
            c.alpha,
        );
        sprite.color = color;
    }
}

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::core::global::Score>()
            .init_resource::<crate::core::global::HighScore>()
            .init_resource::<crate::core::global::FoodsEaten>()
            .init_resource::<crate::core::global::MovementTimer>()
            .add_systems(
                Update,
                animate_food_pulse.in_set(crate::core::global::GameSets::Visuals),
            );
    }
}
