use bevy::prelude::*;

static TILE_SIZE: i32 = 16;

pub fn hex_to_pixel_coords(q: i32, r: i32) -> Vec2 {
    let root3 = 3.0_f32.sqrt();
    let translate_x = TILE_SIZE as f32 / 2.0 * (3.0 / 2.0 * q as f32);
    let translate_y = TILE_SIZE as f32 / root3 * (root3 / 2.0 * q as f32 + root3 * r as f32);
    return Vec2::new(translate_x, translate_y)
}

