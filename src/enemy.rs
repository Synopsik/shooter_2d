use bevy::prelude::*;
use crate::game::Bounds;
use crate::player::Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

const ENEMY_SPEED: f32 = 120.0;

pub fn spawn_enemies(
    mut commands: Commands,
    mut timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    bounds: Res<Bounds>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let side = fastrand::usize(..4);
    let (x, y) = match side {
        0 => (-bounds.half_w - 30.0, fastrand::f32() * bounds.half_h * 2.0 - bounds.half_h),
        1 => (bounds.half_w + 30.0, fastrand::f32() * bounds.half_h * 2.0 - bounds.half_h),
        2 => (fastrand::f32() * bounds.half_w * 2.0 - bounds.half_w, -bounds.half_h - 30.0),
        _ => (fastrand::f32() * bounds.half_w * 2.0 - bounds.half_w, bounds.half_h + 30.0),
    };

    commands.spawn((
        Transform::from_xyz(x, y, 0.0),
        Sprite::from_color(Color::srgb(0.9, 0.2, 0.2), Vec2::splat(22.0)),
        Enemy,
    ));
}


pub fn enemy_seek_player(
    time: Res<Time>,
    mut sets: ParamSet<(
        Query<&Transform, With<Player>>,      // p0: read-only player transform(s)
        Query<&mut Transform, With<Enemy>>,   // p1: write enemy transforms
    )>,
) {
    let player_query = sets.p0();
    let Ok(player_t) = player_query.single() else { return; };
    let player_pos = player_t.translation.truncate();

    for mut et in sets.p1().iter_mut() {
        let dir = (player_pos - et.translation.truncate()).normalize_or_zero();
        let step = dir * ENEMY_SPEED * time.delta_secs();
        et.translation.x += step.x;
        et.translation.y += step.y;
    }
}