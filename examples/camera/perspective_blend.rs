use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(sonolil_app::AppPlugin {
        fps_overlay: false,
        ..default()
    });

    //app.add_plugins(bevy_)

    // .add_plugins(sonolil_basic3d::Basic3DPlugins {
    //     orthographic: true,
    //     draw_grid: true,
    //     dolly: true,
    // })
    app.run();
}
