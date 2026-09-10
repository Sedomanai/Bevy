use crate::modules::focus::CurrentCameraFocus;
use bevy::prelude::*;

/// A trait that extends `Transform` with a convenience method to look at a `CurrentCameraFocus`.
pub trait LookAtExt {
    /// Makes the `Transform` look at the point specified by `CurrentCameraFocus`.
    fn look_at_focus(&mut self, curr: &CurrentCameraFocus);
}

/// Implements `LookAtExt` for `Transform`.
impl LookAtExt for Transform {
    fn look_at_focus(&mut self, curr: &CurrentCameraFocus) {
        // Calls the standard `Transform::look_at` method with the camera's current focus.
        self.look_at(curr.focus, Vec3::Y);
    }
}
