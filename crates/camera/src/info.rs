use bevy::{
    ecs::query::QuerySingleError,
    prelude::*,
    window::{PrimaryWindow, Window},
};

#[derive(Component, Default)]
pub struct CameraMouseInfo {
    last: Option<Vec2>,
    curr: Option<Vec2>,
    hovering: bool,
    pub window: Option<Entity>,
}

impl CameraMouseInfo {
    pub fn viewport_height(&self, windows: &Query<&Window>) -> Option<f32> {
        let entity = self.window?;
        Some(windows.get(entity).ok()?.height())
    }

    pub fn cursor_position(&self, windows: &Query<&Window>) -> Option<Vec2> {
        let entity = self.window?;
        windows.get(entity).ok()?.cursor_position()
    }

    pub fn update_tick(&mut self, windows: &Query<&Window>) {
        if let Some(window) = self.window.and_then(|e| windows.get(e).ok()) {
            self.last = self.curr;
            self.curr = window.cursor_position();
        }
    }

    pub fn move_delta(&self) -> Vec2 {
        match (self.last, self.curr) {
            (Some(last), Some(curr)) => curr - last,
            _ => Vec2::ZERO,
        }
    }
}

pub fn register(app: &mut App) {
    app.add_systems(
        crate::ProcessCameraPluginSchedule,
        set_window_to_all_camera_mouse_info,
    )
    .add_systems(PreUpdate, update_camera_info_mouse_position_to_window);
}

/// Default window of all entities attached with CameraMouseInfo is set to PrimaryWindow
/// Almost most of the time this would be the case.
fn set_window_to_all_camera_mouse_info(
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
fn update_camera_info_mouse_position_to_window(
    mut query: Query<&mut CameraMouseInfo, With<Camera>>,
    windows: Query<&Window>,
) {
    for mut info in query.iter_mut() {
        info.update_tick(&windows);
        //todo!("hover");
    }
}
