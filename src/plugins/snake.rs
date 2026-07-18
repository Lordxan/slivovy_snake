use bevy::prelude::*;

use crate::core::constants::{
    COLOR_SNAKE_BODY, COLOR_SNAKE_HEAD, GRID_HEIGHT, GRID_WIDTH, SEGMENT_SPACING, TILE_SIZE,
};
use crate::core::global::{GameState, MovementTimer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const fn vec2_offset(self) -> Vec2 {
        match self {
            Direction::Up => Vec2::Y,
            Direction::Down => Vec2::new(0.0, -1.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::X,
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn is_opposite(self, other: Self) -> bool {
        self == other.opposite()
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SnakeHead {
    pub direction: Direction,
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SnakeBody;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SnakeSegment;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SegmentColor(pub Color);

#[derive(Resource, Default, Debug)]
pub struct PendingDirection(pub Option<Direction>);

#[derive(Resource, Default, Debug)]
pub struct SnakeMoveState {
    pub dir: Direction,
    pub new_head: Vec2,
    pub old_head_transform: Transform,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SnakeSets {
    Movement,
}

pub fn move_snake_transform(
    mut _commands: Commands,
    mut movement_timer: ResMut<MovementTimer>,
    mut move_state: ResMut<SnakeMoveState>,
    mut pending_direction: ResMut<PendingDirection>,
    mut head_query: Query<(Entity, &mut Transform, &mut SnakeHead), Without<SnakeBody>>,
    mut body_query: Query<(Entity, &mut Transform), With<SnakeBody>>,
    time: Res<Time>,
) {
    if !movement_timer.tick(time.delta()).just_finished() {
        return;
    }

    let (_head_entity, mut head_transform, mut head_dir) = match head_query.single_mut() {
        Ok(e) => e,
        Err(_) => return,
    };

    let head_pos = head_transform.translation.truncate();
    let mut dir = head_dir.direction;
    if let Some(pending) = pending_direction.0.take() {
        if !dir.is_opposite(pending) {
            dir = pending;
        }
    }
    head_dir.direction = dir;
    let offset = dir.vec2_offset() * SEGMENT_SPACING;
    let new_head = head_pos + offset;

    move_state.dir = dir;
    move_state.new_head = new_head;
    move_state.old_head_transform = *head_transform;

    head_transform.translation = new_head.extend(head_transform.translation.z);

    let old_body_transforms = body_query.iter().map(|(_, t)| *t).collect::<Vec<_>>();
    let mut body_iters = body_query.iter_mut();

    if let Some((_, mut first_body)) = body_iters.next() {
        first_body.translation = move_state.old_head_transform.translation;
    }

    let mut transform_iter = old_body_transforms.into_iter();
    while let Some((_, mut body)) = body_iters.next() {
        if let Some(old_t) = transform_iter.next() {
            body.translation = old_t.translation;
        }
    }
}

pub fn spawn_initial_snake(mut commands: Commands) {
    let center = Vec2::new(
        (GRID_WIDTH as f32) / 2.0 * TILE_SIZE,
        (GRID_HEIGHT as f32) / 2.0 * TILE_SIZE,
    );
    let dir = Direction::Right;
    let offset = dir.vec2_offset() * SEGMENT_SPACING;
    let length = 2;

    let head_pos = center;
    let head_color = COLOR_SNAKE_HEAD;
    commands.spawn((
        SegmentColor(head_color),
        SnakeSegment,
        SnakeHead {
            direction: Direction::Right,
        },
        Transform::from_translation(head_pos.extend(0.0)).with_scale(Vec3::splat(1.15)),
        Sprite::from_color(head_color, Vec2::splat(TILE_SIZE * 0.92)),
    ));

    for i in 0..length - 1 {
        let body_pos = center - offset * (i + 1) as f32;
        let color = COLOR_SNAKE_BODY;
        commands.spawn((
            SegmentColor(color),
            SnakeSegment,
            SnakeBody,
            Transform::from_translation(body_pos.extend(0.5)).with_scale(Vec3::splat(0.92)),
            Sprite::from_color(color, Vec2::splat(TILE_SIZE * 0.92)),
        ));
    }
}

pub fn cleanup_snake(mut commands: Commands, snake_segments: Query<Entity, With<SnakeSegment>>) {
    for entity in snake_segments.iter() {
        commands.entity(entity).despawn();
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PendingDirection>()
            .init_resource::<SnakeMoveState>()
            .add_systems(
                OnEnter(GameState::Playing),
                spawn_initial_snake.in_set(SnakeSets::Movement),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_snake)
            .add_systems(Update, move_snake_transform.in_set(SnakeSets::Movement));
    }
}
