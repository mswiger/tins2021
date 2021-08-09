use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

mod game;
mod map;
mod player;
mod util;

struct Camera;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("I Want to Go Home"),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .init_resource::<game::Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(game::GamePlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup_camera)
        .add_system(pan_camera)
        .run();
}


fn setup_camera(mut commands: Commands, mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .update_scale_factor_from_backend(1.0);
    commands
        .spawn_bundle(OrthographicCameraBundle {
            orthographic_projection: bevy::render::camera::OrthographicProjection {
                scale: 0.25,
                ..Default::default()
            },
            ..OrthographicCameraBundle::new_2d()
        })
        .insert(Camera);
}

fn pan_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    game: Res<game::Game>,
) {
    if game.won {
        return;
    }

    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        transform.translation += time.delta_seconds() * direction * 500.;
    }
}
