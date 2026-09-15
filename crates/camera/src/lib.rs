use bevy::prelude::*;

pub mod info;
pub use info::CameraMouseInfo;

mod math;

pub mod projection;
pub use projection::pan_multiplier;
pub use projection::BlendProjection;

pub mod render_target;

pub mod templates;
use sonolil_movement::MovementPlugin;
pub use templates::basic;
pub use templates::blender_cam;

#[cfg(feature = "hub")]
use sonolil_hub::schedule::SpawnTaggedSchedule as SpawnCameraPluginSchedule;
#[cfg(not(feature = "hub"))]
use PreStartup as SpawnCameraPluginSchedule;

#[cfg(feature = "hub")]
use sonolil_hub::schedule::ProcessTaggedSchedule as ProcessCameraPluginSchedule;
#[cfg(not(feature = "hub"))]
use Startup as ProcessCameraPluginSchedule;

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
            CameraPluginTemplate::Blender => {
                app.add_systems(SpawnCameraPluginSchedule, blender_cam::spawn_blender_cams);

                #[cfg(feature = "hub")]
                app.add_systems(Update, blender_cam::update_blender_camera)
            }
        };

        info::register(app);
    }
}
