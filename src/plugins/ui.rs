use crate::core::constants::COLOR_GAME_OVER;
use crate::core::global::{FoodsEaten, HighScore, Score};
use crate::core::utils::{calculate_speed_mult, get_high_score};
use crate::plugins::snake::SnakeSegment;
use bevy::prelude::*;

#[derive(Component)]
pub struct HudScore;
#[derive(Component)]
pub struct HudHighScore;
#[derive(Component)]
pub struct HudLength;
#[derive(Component)]
pub struct HudSpeed;

#[derive(Component)]
pub struct StartScreen;
#[derive(Component)]
pub struct GameOverScreen;

#[derive(Component)]
pub struct HudGroup;
#[derive(Component)]
pub struct StartGroup;
#[derive(Component)]
pub struct GameOverGroup;

const HUD_X: f32 = 10.0;
const HUD_Y: f32 = 10.0;
const HUD_FONT_SIZE: f32 = 20.0;
const TITLE_FONT_SIZE: f32 = 48.0;
const PROMPT_FONT_SIZE: f32 = 24.0;
const SCORE_FONT_SIZE: f32 = 36.0;

fn hud_text_node(left: f32, top: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(left),
        top: Val::Px(top),
        ..default()
    }
}

fn hud_text_color_rich(r: f32, g: f32, b: f32) -> Color {
    Color::srgb(r, g, b)
}

pub fn spawn_hud(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                ..default()
            },
            HudGroup,
        ))
        .with_children(|parent| {
            parent.spawn((
                hud_text_node(HUD_X, HUD_Y),
                HudScore,
                Text::new("Score: 0"),
                TextFont::from_font_size(HUD_FONT_SIZE),
                TextColor(hud_text_color_rich(0.45, 1.0, 0.20)),
            ));

            let high = get_high_score();
            let offset = HUD_FONT_SIZE + 32.0;
            parent.spawn((
                hud_text_node(HUD_X, HUD_Y + offset * 1.0),
                HudHighScore,
                Text::new(format!("Best: {}", high)),
                TextFont::from_font_size(HUD_FONT_SIZE),
                TextColor(hud_text_color_rich(0.85, 0.75, 0.20)),
            ));

            parent.spawn((
                hud_text_node(HUD_X, HUD_Y + offset * 2.0),
                HudLength,
                Text::new("Length: 2"),
                TextFont::from_font_size(HUD_FONT_SIZE),
                TextColor(hud_text_color_rich(0.45, 1.0, 0.20)),
            ));

            parent.spawn((
                hud_text_node(HUD_X, HUD_Y + offset * 3.0),
                HudSpeed,
                Text::new("Speed: 1x"),
                TextFont::from_font_size(HUD_FONT_SIZE),
                TextColor(hud_text_color_rich(0.85, 0.75, 0.20)),
            ));
        });
}

pub fn update_hud(
    score: Res<Score>,
    high_score: Res<HighScore>,
    foods_eaten: Res<FoodsEaten>,
    segment_query: Query<Entity, With<SnakeSegment>>,
    mut texts: Query<
        &mut Text,
        (
            With<HudScore>,
            Without<HudHighScore>,
            Without<HudLength>,
            Without<HudSpeed>,
        ),
    >,
    mut high_texts: Query<&mut Text, (With<HudHighScore>, Without<HudLength>, Without<HudSpeed>)>,
    mut length_texts: Query<&mut Text, (With<HudLength>, Without<HudSpeed>)>,
    mut speed_texts: Query<&mut Text, With<HudSpeed>>,
) {
    if let Ok(mut text) = texts.single_mut() {
        *text = Text::new(format!("Score: {}", score.0));
    }
    if let Ok(mut text) = high_texts.single_mut() {
        *text = Text::new(format!("Best: {}", high_score.0));
    }
    let segment_count = segment_query.iter().count();
    if let Ok(mut text) = length_texts.single_mut() {
        *text = Text::new(format!("Length: {}", segment_count));
    }
    let speed_mult = calculate_speed_mult(foods_eaten.0);
    if let Ok(mut text) = speed_texts.single_mut() {
        *text = Text::new(format!("Speed: {}x", speed_mult));
    }
}

pub fn spawn_start_screen(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(COLOR_GAME_OVER),
            StartGroup,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("SNAKE"),
                TextFont::from_font_size(TITLE_FONT_SIZE),
                TextColor(Color::srgb(0.45, 1.0, 0.20)),
                StartScreen,
            ));

            parent.spawn((
                Text::new("Use WASD or Arrow Keys"),
                TextFont::from_font_size(PROMPT_FONT_SIZE),
                TextColor(Color::srgb(0.6, 0.85, 0.5)),
            ));

            parent.spawn((
                Text::new("Press any key to start"),
                TextFont::from_font_size(PROMPT_FONT_SIZE),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

pub fn setup_start_screen(mut commands: Commands) {
    spawn_start_screen(&mut commands);
}

pub fn cleanup_start_screen(mut commands: Commands, start_group: Query<Entity, With<StartGroup>>) {
    for entity in start_group.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_game_over_screen(commands: &mut Commands, score_value: i32) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(COLOR_GAME_OVER),
            GameOverGroup,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont::from_font_size(TITLE_FONT_SIZE),
                TextColor(Color::srgb(0.95, 0.30, 0.20)),
                GameOverScreen,
            ));

            parent.spawn((
                Text::new(format!("Score: {}", score_value)),
                TextFont::from_font_size(SCORE_FONT_SIZE),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            let high = get_high_score();
            parent.spawn((
                Text::new(format!("Best: {}", high)),
                TextFont::from_font_size(PROMPT_FONT_SIZE),
                TextColor(Color::srgb(0.85, 0.75, 0.20)),
            ));

            parent.spawn((
                Text::new("Press any key to restart"),
                TextFont::from_font_size(PROMPT_FONT_SIZE),
                TextColor(Color::srgb(0.7, 0.9, 0.6)),
            ));
        });
}

pub fn setup_game_over(mut commands: Commands, score: Res<Score>) {
    spawn_game_over_screen(&mut commands, score.0);
}

pub fn cleanup_game_over_screen(
    mut commands: Commands,
    game_over_group: Query<Entity, With<GameOverGroup>>,
) {
    for entity in game_over_group.iter() {
        commands.entity(entity).despawn();
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_hud
                .after(crate::plugins::snake::SnakeSets::Movement)
                .in_set(crate::core::global::GameSets::GameLogic),
        );
    }
}
