use bevy::camera::RenderTarget;
use bevy::prelude::*;
use sonolil_util::*;

mod ui;

pub fn render_target(
    mut camera: Query<&mut RenderTarget, With<tags::MainWorldCamera>>,
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
    app.add_plugins(sonolil_core::CorePlugin {
        reactive_update_mode: true,
        ..default()
    })
    .add_plugins(sonolil_basic2d::Basic2DPlugins {
        draw_grid: true,
        pan_zoom: true,
    })
    .add_plugins(ui::EditorPlugins)
    .add_systems(Update, render_target)
    .run();
}
