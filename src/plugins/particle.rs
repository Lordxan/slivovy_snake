use bevy::prelude::*;

use rand::Rng;

use crate::core::global::GameSets;

#[derive(Resource, Default, Debug, Clone)]
pub struct ParticleBurstEvents(pub Vec<Vec2>);

#[derive(Resource, Default, Debug, Clone)]
pub struct ParticleFlashEvents(pub Vec<Vec2>);

#[derive(Component, Debug, Clone)]
pub struct Particle {
    pub lifetime: std::time::Duration,
    pub remaining: std::time::Duration,
    pub start_scale: f32,
    pub end_scale: f32,
    pub velocity: Vec2,
}

#[derive(Component, Debug, Clone)]
pub struct ParticleSprite(pub Color);

pub fn spawn_burst_particles(
    mut commands: Commands,
    mut burst_events: ResMut<ParticleBurstEvents>,
) {
    let positions = std::mem::take(&mut burst_events.0);
    for pos in positions {
        let n: usize = 12;
        for i in 0..n {
            let angle = 2.0 * std::f32::consts::PI * i as f32 / n as f32
                + (rand::rng().random::<f32>() * 0.3 - 0.15);
            let speed = 40.0 + rand::rng().random_range(0.0..60.0);
            let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);
            let hue = rand::rng().random_range(0.0..0.12);
            let color = Color::srgba(1.0, 0.85 + hue, 0.1 + hue, 1.0);
            commands.spawn((
                Particle {
                    lifetime: std::time::Duration::from_millis(250),
                    remaining: std::time::Duration::from_millis(250),
                    start_scale: 0.5,
                    end_scale: 0.05,
                    velocity: vel,
                },
                ParticleSprite(color),
                Transform::from_translation(pos.extend(1.5)),
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(6.0)),
                    ..default()
                },
            ));
        }
    }
}

pub fn spawn_flash_particles(
    mut commands: Commands,
    mut flash_events: ResMut<ParticleFlashEvents>,
) {
    let positions = std::mem::take(&mut flash_events.0);
    for pos in positions {
        commands.spawn((
            Particle {
                lifetime: std::time::Duration::from_millis(200),
                remaining: std::time::Duration::from_millis(200),
                start_scale: 0.8,
                end_scale: 0.0,
                velocity: Vec2::ZERO,
            },
            ParticleSprite(Color::srgba(1.0, 0.95, 0.3, 0.8)),
            Transform::from_translation(pos.extend(2.0)),
            Sprite {
                color: Color::srgba(1.0, 0.95, 0.3, 0.8),
                custom_size: Some(Vec2::splat(18.0)),
                ..default()
            },
        ));
    }
}

pub fn animate_particles(
    mut commands: Commands,
    mut particles: Query<(
        Entity,
        &mut Particle,
        &mut ParticleSprite,
        &mut Transform,
        &mut Sprite,
    )>,
    time: Res<Time>,
) {
    let dt = time.delta();
    for (entity, mut particle, mut sprite, mut transform, mut sprite_comp) in particles.iter_mut() {
        match particle.remaining.checked_sub(dt) {
            Some(new_remaining) => particle.remaining = new_remaining,
            None => {
                particle.remaining = std::time::Duration::ZERO;
            }
        }
        if particle.remaining <= std::time::Duration::ZERO {
            commands.entity(entity).despawn();
            continue;
        }
        let t = particle.remaining.as_secs_f32() / particle.lifetime.as_secs_f32();
        let scale = particle.start_scale * t + particle.end_scale * (1.0 - t);
        let mut new_transform = transform.clone();
        new_transform.translation += particle.velocity.extend(0.0) * dt.as_secs_f32();
        new_transform.scale = Vec3::splat(scale);
        let alpha = t * t;
        let base_color = sprite.0;
        if let Color::Srgba(c) = base_color {
            sprite.0 = Color::srgba(c.red, c.green, c.blue, alpha);
        }
        if let Color::Srgba(mut c) = sprite_comp.color {
            c.alpha = alpha;
            sprite_comp.color = Color::srgba(c.red, c.green, c.blue, c.alpha);
        }
        transform.clone_from(&new_transform);
    }
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ParticleBurstEvents>()
            .init_resource::<ParticleFlashEvents>()
            .add_systems(Update, spawn_burst_particles.in_set(GameSets::Visuals))
            .add_systems(Update, spawn_flash_particles.in_set(GameSets::Visuals))
            .add_systems(Update, animate_particles.in_set(GameSets::Visuals));
    }
}
