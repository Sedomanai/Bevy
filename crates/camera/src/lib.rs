use bevy::prelude::*;

mod _util;
pub use _util::projection::BlendProjection;

mod systems;

mod components;
pub use components::*;

use sonolil_movement::MovementPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraPluginTemplate {
    Blender,
}

#[derive(Debug, Deref, DerefMut)]
pub struct CameraPlugin(pub CameraPluginTemplate);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<MovementPlugin>() {
            app.add_plugins(MovementPlugin);
        }

        match self.0 {
            CameraPluginTemplate::Blender => systems::setup_blender_template(app),
        };

        systems::register_systems(app);
    }
}
