use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource)]
pub struct Bounds {
    pub half_w: f32,
    pub half_h: f32,
}

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Resource)]
pub struct PlayerHealth(pub i32);

#[derive(Component)]
pub struct CounterText;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Bullets: 0 | Enemies: 0"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        CounterText,
    ));
}

pub fn update_counter_text(
    bullet_query: Query<&crate::bullet::Bullet>,
    enemy_query: Query<&crate::enemy::Enemy>,
    mut text_query: Query<&mut Text, With<CounterText>>,
) {
    let bullet_count = bullet_query.iter().count();
    let enemy_count = enemy_query.iter().count();
    
    if let Ok(mut text) = text_query.single_mut() {
        *text = Text::new(format!("Bullets: {} | Enemies: {}", bullet_count, enemy_count));
    }
}

pub fn update_window_title(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    score: Res<Score>,
    hp: Res<PlayerHealth>,
) {
    if score.is_changed() || hp.is_changed() {
        if let Ok(mut w) = windows.single_mut() {
            w.title = format!("Minimal TDS | Score: {} HP: {}", score.0, hp.0);
        }
    }
}