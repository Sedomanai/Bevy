use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    sonolil_core::init_editor_plugins(&mut app);
    sonolil_basic2d::init_plugins(&mut app);
    app.run();
}
