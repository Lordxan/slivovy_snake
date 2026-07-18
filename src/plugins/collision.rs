use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;

use crate::core::constants::{SEGMENT_SPACING, WORLD_BOTTOM, WORLD_LEFT, WORLD_RIGHT, WORLD_TOP};
use crate::core::global::{GameSets, GameState};
use crate::plugins::snake::{SnakeBody, SnakeHead, SnakeSets};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CollisionSets {
    Detection,
}

fn world_aabb() -> Aabb2d {
    Aabb2d::new(
        Vec2::new(WORLD_LEFT, WORLD_BOTTOM),
        Vec2::new(WORLD_RIGHT, WORLD_TOP),
    )
}

pub fn check_boundary_collision(
    mut next_state: ResMut<NextState<GameState>>,
    head_query: Query<&Transform, (With<SnakeHead>, Changed<Transform>)>,
) {
    let head_transform = match head_query.single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let head_pos = head_transform.translation.truncate();
    let head_radius = SEGMENT_SPACING * 0.5;
    let head_circle = BoundingCircle::new(head_pos, head_radius);

    if !head_circle.intersects(&world_aabb()) {
        next_state.set(GameState::GameOver);
    }
}

pub fn check_self_collision(
    mut next_state: ResMut<NextState<GameState>>,
    head_query: Query<&Transform, (With<SnakeHead>, Changed<Transform>)>,
    body_query: Query<&Transform, (With<SnakeBody>, Changed<Transform>)>,
) {
    let head_transform = match head_query.single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let head_pos = head_transform.translation.truncate();
    let head_radius = SEGMENT_SPACING * 0.5;
    let head_circle = BoundingCircle::new(head_pos, head_radius);

    for body_transform in body_query.iter() {
        let body_pos = body_transform.translation.truncate();

        let body_radius = SEGMENT_SPACING * 0.5;
        let body_circle = BoundingCircle::new(body_pos, body_radius);

        if head_circle.intersects(&body_circle) {
            next_state.set(GameState::GameOver);
            return;
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_boundary_collision
                    .after(SnakeSets::Movement)
                    .in_set(CollisionSets::Detection),
                check_self_collision
                    .after(SnakeSets::Movement)
                    .in_set(CollisionSets::Detection),
            )
                .in_set(GameSets::GameEvents),
        );
    }
}
