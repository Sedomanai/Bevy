use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(sonolil_app::AppPlugin {
        fps_overlay: true,
        ..default()
    })
    .add_plugins(bevy::dev_tools::infinite_grid::InfiniteGridPlugin)
    .run();
}
