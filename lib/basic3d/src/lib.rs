use bevy::prelude::*;
use bevy_dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings};

mod cam;

/// Initializes the basic 3D plugins for the application.
/// Adds `InfiniteGridPlugin` and registers `setup` and `dolly_camera_system`.
pub fn init_plugins(app: &mut bevy::app::App) {
    app.add_plugins(InfiniteGridPlugin)
        .add_systems(Startup, (setup, cam::setup))
        .add_systems(Update, cam::dolly_camera_system);
}

/// Sets up the infinite grid for 3D debugging.
/// Spawns an `InfiniteGrid` entity with default settings.
fn setup(mut commands: Commands) {
    commands.spawn((InfiniteGrid, InfiniteGridSettings::default()));
}
