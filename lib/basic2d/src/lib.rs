use bevy::prelude::*;

mod cam;
mod grid2d;

pub fn init_plugins(app: &mut bevy::app::App) {
    grid2d::init_plugins(app);
    // Add the InfiniteGridPlugin for 3D debugging with a visual grid.
    app.add_systems(Startup, (cam::setup, grid2d::setup))
        .add_systems(Update, cam::pan_zoom_camera_system);
}
