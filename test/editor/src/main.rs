use bevy::camera::RenderTarget;
use bevy::prelude::*;
use sonolil_core::MainCamera;

mod ui;

pub fn render_target(
    mut camera: Query<&mut RenderTarget, With<MainCamera>>,
    image_handle: ResMut<ui::view::ViewportImage>,
) {
    if image_handle.is_changed() {
        if let Ok(mut target) = camera.single_mut() {
            *target = RenderTarget::Image(image_handle.clone().into());
        }
    }
}

fn main() {
    let mut app = App::new();
    sonolil_core::init_editor_plugins(&mut app);
    sonolil_basic2d::init_plugins(&mut app);
    app.add_plugins(ui::EditorPlugins);
    app.add_systems(Update, render_target);
    app.run();
}
