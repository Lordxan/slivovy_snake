use bevy::math::bounding::{BoundingCircle, IntersectsVolume};
use bevy::prelude::*;

use crate::core::constants::{COLOR_FOOD, COLOR_SNAKE_BODY, SEGMENT_SPACING, TILE_SIZE};
use crate::core::global::{FoodsEaten, GameSets, GameState, Score};
use crate::core::utils::random_food_position;
use crate::plugins::snake::{SnakeBody, SnakeHead, SnakeSegment};

#[derive(Resource, Default, Debug, Clone)]
pub struct EatEvents(pub Vec<Vec2>);

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct FoodEntity;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FoodSets {
    Collision,
}

pub fn spawn_food(
    mut commands: Commands,
    food_query: Query<Entity, With<FoodEntity>>,
    snake_segments: Query<&Transform, (With<SnakeSegment>, Without<SnakeHead>)>,
) {
    if food_query.single().is_ok() {
        return;
    }

    let segment_positions: Vec<Vec2> = snake_segments
        .iter()
        .map(|t| t.translation.truncate())
        .collect();

    let food_pos = random_food_position(&segment_positions);
    commands.spawn((
        Sprite {
            color: COLOR_FOOD,
            custom_size: Some(Vec2::splat(TILE_SIZE * 0.7)),
            ..default()
        },
        FoodEntity,
        Transform::from_translation(food_pos.extend(1.0)).with_scale(Vec3::splat(0.85)),
    ));
}

pub fn check_food_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut foods_eaten: ResMut<FoodsEaten>,
    mut eat_events: ResMut<EatEvents>,
    food_query: Query<(Entity, &Transform), With<FoodEntity>>,
    head_query: Query<(&Transform, &SnakeHead), Changed<Transform>>,
    snake_segments: Query<&Transform, (With<SnakeSegment>, Without<SnakeHead>)>,
) {
    let (head_transform, _head_dir) = match head_query.single() {
        Ok(t) => t,
        Err(_) => return,
    };
    let head_pos = head_transform.translation.truncate();
    let head_radius = SEGMENT_SPACING * 0.5;
    let head_circle = BoundingCircle::new(head_pos, head_radius);

    let foods_to_eat: Vec<(Entity, Vec2)> = food_query
        .iter()
        .filter_map(|(entity, t)| {
            let food_pos = t.translation.truncate();
            let food_radius = SEGMENT_SPACING * 0.5;
            let food_circle = BoundingCircle::new(food_pos, food_radius);

            if head_circle.intersects(&food_circle) {
                Some((entity, food_pos))
            } else {
                None
            }
        })
        .collect();

    for (food_entity, food_pos) in foods_to_eat {
        commands.entity(food_entity).despawn();

        score.0 += 1;
        foods_eaten.0 += 1;
        eat_events.0.push(food_pos);

        let mut segment_positions: Vec<Vec2> = snake_segments
            .iter()
            .map(|t| t.translation.truncate())
            .collect();
        segment_positions.push(head_pos);
        let new_food_pos = random_food_position(&segment_positions);
        commands.spawn((
            Sprite {
                color: COLOR_FOOD,
                custom_size: Some(Vec2::splat(TILE_SIZE * 0.7)),
                ..default()
            },
            FoodEntity,
            Transform::from_translation(new_food_pos.extend(1.0)).with_scale(Vec3::splat(0.85)),
        ));

        let tail_pos = snake_segments
            .iter()
            .max_by(|a, b| {
                let dist_a = a.translation.truncate().distance(head_pos);
                let dist_b = b.translation.truncate().distance(head_pos);
                dist_a
                    .partial_cmp(&dist_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|t| t.translation.truncate())
            .unwrap_or(head_pos);

        commands.spawn((
            SnakeSegment,
            SnakeBody,
            Transform::from_translation(tail_pos.extend(0.0)).with_scale(Vec3::splat(0.92)),
            Sprite::from_color(COLOR_SNAKE_BODY, Vec2::splat(TILE_SIZE * 0.92)),
        ));
    }
}

pub fn despawn_food_on_game_over(
    mut commands: Commands,
    food_query: Query<(Entity, &Transform), With<FoodEntity>>,
) {
    let foods_to_despawn: Vec<(Entity, Vec2)> = food_query
        .iter()
        .map(|(entity, t)| (entity, t.translation.truncate()))
        .collect();

    for (entity, _) in foods_to_despawn {
        commands.entity(entity).despawn();
    }
}

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EatEvents>()
            .add_systems(
                OnEnter(GameState::Playing),
                spawn_food.in_set(GameSets::GameEvents),
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                despawn_food_on_game_over.in_set(GameSets::Visuals),
            )
            .add_systems(Update, check_food_collision.in_set(FoodSets::Collision));
    }
}
