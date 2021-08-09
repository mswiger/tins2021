use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;

use super::map::*;
use super::util::*;
use super::Camera;

pub struct Player;

pub struct Cursor;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(cursor_init)
            .add_system(movement_system)
            .add_system(cursor_system);
    }
}

fn cursor_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load("cursor.png").into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)),
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            ..Default::default()
        })
        .insert(Cursor);
}

fn movement_system(
    windows: Res<Windows>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera>>,
    walkable_tile_query: Query<&Tile, With<Walkable>>,
    mut tile_query: Query<(&Tile, &mut Visible)>,
) {
    let window = windows.get_primary().unwrap();
    let mut player_transform = player_query
        .single_mut()
        .expect("There should only be one player.");

    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Some(mouse_pos) = window.cursor_position() {
            let (cam_transform, cam_projection) = camera_query.single().unwrap();
            let mouse_world_pos =
                window_to_world_coords(&window, &cam_transform, &cam_projection, mouse_pos);

            let cur_player_coords =
                Hex::from_pixel_coords(&Vec2::from(player_transform.translation));
            let mouse_tile_coords = Hex::from_pixel_coords(&mouse_world_pos);
            let dest_tile = walkable_tile_query
                .iter()
                .find(|t| t.hex == mouse_tile_coords);

            if let Some(dt) = dest_tile {
                if cur_player_coords.distance_to(&mouse_tile_coords) == 1 {
                    let player_dest = mouse_tile_coords.to_pixel_coords();
                    player_transform.translation.x = player_dest.x;
                    player_transform.translation.y = player_dest.y;

                    tile_query
                        .iter_mut()
                        .filter(|(tile, _)| tile.hex.distance_to(&dt.hex) <= 1)
                        .for_each(|(_, mut visible)| visible.is_visible = true);
                }
            }
        }
    }
}

fn cursor_system(
    windows: Res<Windows>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut cursor_query: Query<
        (&mut Transform, &mut Visible),
        (With<Cursor>, Without<Camera>, Without<Player>),
    >,
    mut cursor_event_reader: EventReader<CursorMoved>,
    mouse_buttons: Res<Input<MouseButton>>,
    tile_query: Query<&Tile, With<Walkable>>,
) {
    let (mut cursor_transform, mut cursor_visible) = cursor_query
        .single_mut()
        .expect("There should only be one cursor.");
    let window = windows.get_primary().unwrap();
    if let Some(ev) = cursor_event_reader.iter().last() {
        let mouse_pos = ev.position;
        let (cam_transform, cam_projection) = camera_query.single().unwrap();
        let mouse_world_pos =
            window_to_world_coords(&window, &cam_transform, &cam_projection, mouse_pos);
        let mouse_tile_coords = Hex::from_pixel_coords(&mouse_world_pos);
        let dest_tile = tile_query.iter().find(|t| t.hex == mouse_tile_coords);

        if let Some(dt) = dest_tile {
            let player_transform = player_query
                .single()
                .expect("There should only be one player.");
            let cur_player_coords =
                Hex::from_pixel_coords(&Vec2::from(player_transform.translation));
            if dt.hex.distance_to(&cur_player_coords) == 1 {
                let dest = dt.hex.to_pixel_coords();
                cursor_transform.translation.x = dest.x;
                cursor_transform.translation.y = dest.y;
                cursor_visible.is_visible = true;
                return;
            }
        }
        cursor_visible.is_visible = false;
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        cursor_visible.is_visible = false;
    }
}

fn window_to_world_coords(
    window: &Window,
    cam_transform: &Transform,
    cam_projection: &OrthographicProjection,
    coords: Vec2,
) -> Vec2 {
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let scale = cam_projection.scale;
    let transformed_coords = (coords - window_size / 2.0) * scale;
    Vec2::new(
        cam_transform.translation.x + transformed_coords.x,
        cam_transform.translation.y + transformed_coords.y,
    )
}
