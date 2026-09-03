use bevy::prelude::*;

mod cam;
mod grid2d;

/// Initializes 2D specific plugins for the application, including camera and grid systems.
pub fn init_plugins(app: &mut bevy::app::App) {
    grid2d::init_plugins(app);
    // Add the InfiniteGridPlugin for 3D debugging with a visual grid.
    // Configures startup systems to initialize the 2D camera and grid, and registers the camera pan/zoom system for continuous updates.
    app.add_systems(Startup, (cam::setup, grid2d::setup))
        .add_systems(Update, cam::pan_zoom_camera_system);
}
