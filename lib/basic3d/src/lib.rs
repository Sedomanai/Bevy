use bevy::prelude::*;
use bevy_dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings};

mod cam;

pub fn init_plugins(app: &mut bevy::app::App) {
    // Add the InfiniteGridPlugin for 3D debugging with a visual grid.
    app.add_plugins(InfiniteGridPlugin)
        // Register the setup_debug_grid_3d system to run once at application startup.
        .add_systems(Startup, (setup, cam::setup))
        .add_systems(Update, cam::dolly_camera_system);
}

fn setup(mut commands: Commands) {
    commands.spawn((InfiniteGrid, InfiniteGridSettings::default()));
}
