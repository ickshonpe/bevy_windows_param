//! Reports the position of the cursor.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(report_cursor_position)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn report_cursor_position(windows: bevy_windows_param::Windows) {
    if let Some(cursor_position) = windows.ui_cursor_position() {
        println!("{}", cursor_position);
    }
}
