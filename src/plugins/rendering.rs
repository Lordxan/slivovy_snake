use bevy::prelude::*;

use crate::core::constants::{COLOR_BG, COLOR_GRID_LINE, GRID_HEIGHT, GRID_WIDTH, TILE_SIZE};

pub fn grid_world_size() -> Vec2 {
    Vec2::new(
        GRID_WIDTH as f32 * TILE_SIZE,
        GRID_HEIGHT as f32 * TILE_SIZE,
    )
}

pub fn grid_center() -> Vec2 {
    let sz = grid_world_size();
    Vec2::new(sz.x / 2.0, sz.y / 2.0)
}

pub struct RenderingPlugin;

pub fn spawn_startup(mut commands: Commands) {
    let cam_size = grid_world_size();
    let cam_pos = grid_center();

    let cam_center = cam_pos;
    commands.spawn((
        Camera2d,
        Transform::from_translation(cam_center.extend(100.0)),
        IsDefaultUiCamera,
    ));

    commands.insert_resource(ClearColor(COLOR_BG));

    let bg_size = cam_size + Vec2::splat(8.0);
    commands.spawn((
        Sprite::from_color(COLOR_BG, bg_size),
        Transform::from_translation(cam_pos.extend(-1.0)),
    ));

    let grid_thickness = 0.5;
    for i in 0..=GRID_HEIGHT {
        let y = i as f32 * TILE_SIZE;
        commands.spawn((
            Sprite::from_color(COLOR_GRID_LINE, Vec2::new(cam_size.x, grid_thickness)),
            Transform::from_translation(
                Vec2::new(cam_size.x / 2.0, y + TILE_SIZE / 2.0).extend(-0.5),
            ),
        ));
    }
    for j in 0..=GRID_WIDTH {
        let x = j as f32 * TILE_SIZE;
        commands.spawn((
            Sprite::from_color(COLOR_GRID_LINE, Vec2::new(grid_thickness, cam_size.y)),
            Transform::from_translation(
                Vec2::new(x + TILE_SIZE / 2.0, cam_size.y / 2.0).extend(-0.5),
            ),
        ));
    }
}

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_startup);
    }
}
