use bevy::{
    prelude::*,
    window::{PrimaryWindow, Window},
};

use crate::components::CameraMouseInfo;

/// Default window of all entities attached with CameraMouseInfo is set to PrimaryWindow
/// Almost most of the time this would be the case.
pub fn set_window_to_all_camera_mouse_info(
    mut query: Query<&mut CameraMouseInfo, With<Camera>>,
    window: Query<Entity, (With<Window>, With<PrimaryWindow>)>,
) {
    let Ok(window) = window.single() else {
        return;
    };

    for mut info in query.iter_mut() {
        info.window = Some(window);
    }
}

/// Update all attached with CameraMouseInfo is set to PrimaryWindow
/// Almost most of the time this would be the case.
pub fn update_camera_info_mouse_position_to_window(
    mut query: Query<&mut CameraMouseInfo, With<Camera>>,
    windows: Query<&Window>,
) {
    for mut info in query.iter_mut() {
        info.update_tick(&windows);
        //todo!("hover");
    }
}
