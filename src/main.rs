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
        .add_plugin(map::MapPlugin)
        .add_startup_system(setup_camera)
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

