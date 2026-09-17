use bevy::prelude::*;

mod _blender_system;
mod _info_system;
mod _projection_system;

#[cfg(feature = "hub")]
use sonolil_hub::schedule::SpawnTaggedSchedule as SpawnCameraPluginSchedule;
#[cfg(not(feature = "hub"))]
use PreStartup as SpawnCameraPluginSchedule;

#[cfg(feature = "hub")]
use sonolil_hub::schedule::ProcessTaggedSchedule as ProcessCameraPluginSchedule;
#[cfg(not(feature = "hub"))]
use Startup as ProcessCameraPluginSchedule;

#[cfg(feature = "hub")]
use sonolil_hub::schedule::SonolilInputSchedule as InputCameraPluginSchedule;
#[cfg(not(feature = "hub"))]
use PreUpdate as InputCameraPluginSchedule;

use crate::components;

pub fn setup_blender_template(app: &mut App) {
    app.add_systems(SpawnCameraPluginSchedule, move |mut commands: Commands| {
        commands.spawn((
            components::BlenderCameraBundle::new(Vec3::new(7.358, 4.958, 7.358), Vec3::ZERO),
            #[cfg(feature = "hub")]
            (sonolil_hub::tags::EngineCamera),
        ));
    })
    .add_systems(
        InputCameraPluginSchedule,
        _blender_system::update_blender_camera,
    );
}

pub fn register_systems(app: &mut App) {
    app.add_systems(
        ProcessCameraPluginSchedule,
        _info_system::set_window_to_all_camera_mouse_info,
    )
    .add_systems(
        InputCameraPluginSchedule,
        _info_system::update_camera_info_mouse_position_to_window,
    );
}
