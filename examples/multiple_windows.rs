//! Reports the position of the cursor across multiple windows.

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef};

fn main() {
    App::new()
        // By default, a primary window gets spawned by `WindowPlugin`, contained in `DefaultPlugins`
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Primary Window".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup)
        .add_system(report_cursor_position)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    for (window_title, window_color) in [
        ("Second Window", Color::YELLOW),
        ("Third Window", Color::RED),
    ] {
        let window = commands
            .spawn(Window {
                title: window_title.into(),
                ..Default::default()
            })
            .id();
        commands.spawn(Camera2dBundle {
            camera: Camera {
                target: RenderTarget::Window(WindowRef::Entity(window)),
                ..Default::default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(window_color),
            },
            ..Default::default()
        });
    }
}

fn report_cursor_position(windows: bevy_windows_param::Windows) {
    if let Some(cursor_position) = windows.ui_cursor_position() {
        println!("{}", cursor_position);
    }
}
