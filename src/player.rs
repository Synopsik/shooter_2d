use bevy::prelude::*;
use crate::bullet::{Bullet, Velocity, BULLET_SPEED};
use crate::game::Bounds;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct FireCooldown(pub Timer);

const PLAYER_SPEED: f32 = 320.0;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh_handle = meshes.add(RegularPolygon::new(12.0, 3)); // radius 12, triangle
    let material_handle = materials.add(ColorMaterial::from(Color::WHITE));
    commands.spawn((
        Mesh2d(mesh_handle.into()),
        MeshMaterial2d(material_handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
    ));
}

pub fn move_player(
    kb: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    bounds: Res<Bounds>,
    mut q: Query<&mut Transform, With<Player>>,
) {
    let mut t = q.single_mut().unwrap();
    let mut dir = Vec2::ZERO;

    if kb.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if kb.pressed(KeyCode::KeyS) { dir.y -= 1.0; }
    if kb.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if kb.pressed(KeyCode::KeyD) { dir.x += 1.0; }
    if dir.length_squared() > 0.0 { dir = dir.normalize(); }

    t.translation.x += dir.x * PLAYER_SPEED * time.delta_secs();
    t.translation.y += dir.y * PLAYER_SPEED * time.delta_secs();

    t.translation.x = t.translation.x.clamp(-bounds.half_w, bounds.half_w);
    t.translation.y = t.translation.y.clamp(-bounds.half_h, bounds.half_h);
}

pub fn aim_with_mouse(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut q_player: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = q_player.single_mut() else {return;};
    let Some(cursor_world) = cursor_world_position(windows, camera_q) else {return;};

    let player_pos = transform.translation.truncate();
    let dir = cursor_world - player_pos;

    let angle = dir.y.atan2(dir.x);
    transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
}

pub fn handle_shooting(
    mut commands: Commands,
    mut cd: ResMut<FireCooldown>,
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    q_player: Query<&Transform, With<Player>>,
) {
    cd.0.tick(time.delta());
    if !mouse.pressed(MouseButton::Left) || !cd.0.is_finished() {
        return;
    }

    let Ok(transform) = q_player.single() else { return; };
    let facing_dir = transform.rotation * Vec3::Y;
    let spawn_pos = transform.translation + facing_dir * 18.0;

    commands.spawn((
        Transform::from_translation(spawn_pos).with_rotation(transform.rotation),
        Sprite::from_color(Color::BLACK, Vec2::new(6.0, 14.0)),
        Bullet,
        Velocity(facing_dir.truncate() * BULLET_SPEED),
    ));
}

pub fn cursor_world_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let (camera, cam_transform) = camera_q.single().unwrap();
    let window = windows.single().unwrap();

    let Some(cursor_pos) = window.cursor_position() else { return None };
    camera.viewport_to_world_2d(cam_transform, cursor_pos).ok()
}