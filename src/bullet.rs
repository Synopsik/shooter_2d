use bevy::prelude::*;
use crate::game::Bounds;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub const BULLET_SPEED: f32 = 700.0;

pub fn move_bullets(
    time: Res<Time>,
    mut q: Query<(&Velocity, &mut Transform),
        With<Bullet>>
) {
    for (v, mut t) in &mut q {
        t.translation.x += v.0.x * time.delta_secs();
        t.translation.y += v.0.y * time.delta_secs();
    }
}

pub fn despawn_offscreen<T: Component>(
    mut commands: Commands,
    bounds: Res<Bounds>,
    q: Query<(Entity, &Transform), With<T>>,
) {
    for (e, t) in &q {
        if t.translation.x.abs() > bounds.half_w + 50.0 ||
            t.translation.y.abs() > bounds.half_h + 50.0
        {
            commands.entity(e).despawn();
        }
    }
}