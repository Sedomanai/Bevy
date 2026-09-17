use bevy::{prelude::*, window::Window};

#[derive(Component, Default)]
pub struct CameraMouseInfo {
    last: Option<Vec2>,
    curr: Option<Vec2>,
    //hovering: bool,
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
