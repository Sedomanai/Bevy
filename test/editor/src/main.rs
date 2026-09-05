use bevy::prelude::*;
mod ui;

fn main() {
    let mut app = App::new();
    app.add_plugins(sonolil_core::CorePlugin {
        reactive_update_mode: true,
        ..default()
    })
    .add_plugins(sonolil_basic2d::Basic2DPlugins {
        draw_grid: true,
        pan_zoom: true,
    })
    .add_plugins(ui::EditorPlugins)
    .run();
}
