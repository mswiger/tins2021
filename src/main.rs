use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

mod game;
mod map;
mod player;
mod util;

struct MainMenuUI;

struct Camera;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("I Want to Go Home"),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(26. / 255., 28. / 255., 44. / 255.)))
        .init_resource::<game::Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(game::GamePlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_music)
        .add_system(pan_camera)
        .add_startup_system(setup_menu)
        .add_system(handle_menu_input)
        .run();
}

fn setup_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music: Handle<AudioSource> =
        asset_server.load("Peer Gynt Suite no. 1, Op. 46 - I. Morning Mood.mp3");
    audio.play(music);
}

fn setup_menu(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Find the portal to go home. For another exciting \
                        \"technically a game,\" check out github.com/mswiger/curio",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::rgb(177. / 255., 62. / 255., 83. / 255.),
                            font: asset_server.load("FiraSans-Bold.ttf"),
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(MainMenuUI);
        })
        .insert(MainMenuUI);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "I Want to Go Home",
                        TextStyle {
                            font_size: 100.0,
                            color: Color::rgb(177. / 255., 62. / 255., 83. / 255.),
                            font: asset_server.load("FiraSans-Bold.ttf"),
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Bottom,
                            ..Default::default()
                        },
                    ),
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(MainMenuUI);
        })
        .insert(MainMenuUI);
}

fn handle_menu_input(
    mut commands: Commands,
    mut key_event_reader: EventReader<KeyboardInput>,
    mut mouse_event_reader: EventReader<MouseButtonInput>,
    main_menu_ui_query: Query<Entity, With<MainMenuUI>>,
) {
    let mut button_pressed = false;
    if let Some(_) = mouse_event_reader.iter().last() {
        button_pressed = true;
    }
    if let Some(_) = key_event_reader.iter().last() {
        button_pressed = true;
    }
    if button_pressed {
        for menu_entity in main_menu_ui_query.iter() {
            commands.entity(menu_entity).despawn();
        }
    }
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
