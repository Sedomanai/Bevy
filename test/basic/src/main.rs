use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    sonolil_core::init_core_plugins(&mut app);
    sonolil_basic3d::init_plugins(&mut app);
    app.run();
}
