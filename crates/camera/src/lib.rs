use bevy::prelude::*;

#[cfg(feature = "hub")]
use sonolil_hub::*;

use sonolil_movement::{orbital::Orbiter, tracker::Tracker};

pub mod math;
pub mod projection;
pub mod render_target;

#[cfg(feature = "hub")]
use sonolil_hub::schedule::SpawnTaggedSchedule as SpawnCameraPluginSchedule;

#[cfg(not(feature = "hub"))]
use PreStartup as SpawnCameraPluginSchedule;

#[derive(Bundle)]
pub struct SonolilCameraBundle {
    transform: Transform,
    camera: Camera,
    tracker: Tracker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraPluginTemplate {
    Blender,
}

pub struct CameraPlugin {
    pub starting_point: CameraPluginTemplate,
}

impl Default for CameraPlugin {
    fn default() -> Self {
        Self {
            starting_point: CameraPluginTemplate::Blender,
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        match self.starting_point {
            CameraPluginTemplate::Blender => {
                app.add_systems(SpawnCameraPluginSchedule, spawn_blender_cam);
            }
        };
    }
}

fn spawn_blender_cam(commands: Commands) {
    let blender_camera_pos = Vec3::new(7.358, 4.958, 7.358);

    spawn_sonolil_camera(
        commands,
        SonolilCameraBundle {
            transform: Transform::from_translation(blender_camera_pos)
                .looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera::default(),
            tracker: Tracker::default(),
        },
        #[cfg(feature = "hub")]
        tags::EngineCamera,
        #[cfg(not(feature = "hub"))]
        (),
    );
}

pub fn spawn_sonolil_camera<T>(
    mut commands: Commands,
    sonolil_cam_bundle: SonolilCameraBundle,
    extra_bundle: T,
) -> Entity
where
    T: Bundle,
{
    let proj = Projection::custom(projection::BlendProjection::new(
        0.0,
        sonolil_cam_bundle
            .transform
            .translation
            .distance(Vec3::ZERO),
    ));
    commands
        .spawn((sonolil_cam_bundle, extra_bundle, proj))
        .id()
}
