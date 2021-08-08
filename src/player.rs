use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;

use super::util::*;
use super::Camera;

pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_system);
    }
}

fn movement_system(
    windows: Res<Windows>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera>>,
) {
    let window = windows.get_primary().unwrap();
    let mut transform = player_query
        .single_mut()
        .expect("There should only be one player.");

    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Some(mouse_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let (cam_transform, cam_projection) = camera_query.single().unwrap();
            let scale = cam_projection.scale;
            let transformed_mouse_pos = (mouse_pos - window_size / 2.0) * scale;
            let world_pos = Vec2::new(
                cam_transform.translation.x + transformed_mouse_pos.x,
                cam_transform.translation.y + transformed_mouse_pos.y,
            );

            let tile_coords = Hex::from_pixel_coords(world_pos).to_pixel_coords();
            transform.translation = Vec3::new(tile_coords.x, tile_coords.y, transform.translation.z);
        }
    }
}
