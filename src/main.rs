mod game;
mod player;
mod bullet;
mod enemy;
mod collision;

use bevy::prelude::*;
use game::{Bounds, Score, PlayerHealth};
use crate::enemy::EnemySpawnTimer;
use crate::player::FireCooldown;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Minimal TDS".into(),
                resolution: (960, 540).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Bounds { half_w: 480.0, half_h: 270.0 })
        .insert_resource(Score(0))
        .insert_resource(PlayerHealth(3))
        .insert_resource(FireCooldown(Timer::from_seconds(0.12, TimerMode::Repeating)))
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(0.8, TimerMode::Repeating)))
        .add_systems(Startup, (game::setup_camera, game::setup_ui, player::spawn_player))
        .add_systems(Update, (
            // Player
            player::move_player,
            player::aim_with_mouse,
            player::handle_shooting,
            // Bullet
            bullet::move_bullets,
            bullet::despawn_offscreen::<bullet::Bullet>,
            // Enemy
            enemy::spawn_enemies,
            enemy::enemy_seek_player,
            // Collisions
            collision::bullet_enemy_collisions,
            collision::enemy_player_collisions,
            // System
            bullet::despawn_offscreen::<enemy::Enemy>,
            game::update_counter_text,
            game::update_window_title,
        ))
        .run();
}
