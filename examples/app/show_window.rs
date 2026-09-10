use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(sonolil_app::AppPlugin {
        fps_overlay: true,
        ..default()
    })
    // .add_plugins(sonolil_basic3d::Basic3DPlugins {
    //     orthographic: true,
    //     draw_grid: true,
    //     dolly: true,
    // })
    .run();
}
