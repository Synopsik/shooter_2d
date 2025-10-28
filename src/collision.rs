use bevy::prelude::*;
use crate::bullet::Bullet;
use crate::enemy::Enemy;
use crate::game::{PlayerHealth, Score};
use crate::player::Player;

fn radius_for(_transform: &Transform) -> f32 {
    12.0
}

pub fn bullet_enemy_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    q_bullets: Query<(Entity, &Transform), With<Bullet>>,
    q_enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (b_entity, b_t) in &q_bullets {
        let b_pos = b_t.translation.truncate();
        let b_r = radius_for(b_t);

        for (e_entity, e_t) in &q_enemies {
            let e_pos = e_t.translation.truncate();
            let e_r = radius_for(e_t);

            if b_pos.distance(e_pos) < (b_r + e_r) {
                commands.entity(b_entity).despawn();
                commands.entity(e_entity).despawn();
                score.0 = score.0.saturating_add(1);
                // break so a single bullet can't pop multiple in the same frame
                break;
            }
        }
    }
}

pub fn enemy_player_collisions(
    mut commands: Commands,
    mut hp: ResMut<PlayerHealth>,
    q_player: Query<&Transform, With<Player>>,
    q_enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    let Ok(p_t) = q_player.single() else { return; };
    let p_pos = p_t.translation.truncate();
    let p_r = radius_for(p_t);

    for (e_entity, e_t) in &q_enemies {
        let e_pos = e_t.translation.truncate();
        let e_r = radius_for(e_t);

        if p_pos.distance(e_pos) < (p_r + e_r) {
            commands.entity(e_entity).despawn();
            hp.0 = (hp.0 - 1).max(0);
        }
    }

    // Optional: lightweight “game over” handling
    // if hp.0 == 0 { /* reset or show message; we’ll keep it minimal here */ }
}