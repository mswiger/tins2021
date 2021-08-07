use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

mod map;

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
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(map::MapPlugin)
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
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let scale = transform.scale.x;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            let scale = scale + 0.1;
            transform.scale = Vec3::splat(scale);
        }

        if keyboard_input.pressed(KeyCode::X) {
            let scale = scale - 0.1;
            transform.scale = Vec3::splat(scale);
        }

        if transform.scale.x < 1.0 {
            transform.scale = Vec3::splat(1.0)
        }

        transform.translation += time.delta_seconds() * direction * 500.;
    }
}
