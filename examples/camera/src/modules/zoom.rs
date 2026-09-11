use bevy::{math::NormedVectorSpace, prelude::*};

/// Desired pint in 3D space that the camera should focus on, which can be a static `Vec3` or an `Entity`.
#[derive(Component, Default, DerefMut, Deref, Copy, Clone)]
#[require(CurrentCameraZoom)]
pub struct CameraZoom(f32);

impl CameraZoom {
    // Returns delta
    pub fn zoom(&mut self, delta: f32) {
        if delta != 0.0f32 {
            self.0 -= delta * self.0 * 0.1;
            self.0 = self.max(0.1); // Prevent going through zero
        }
    }
}

/// Current point in 3D space that the camera is focused on.
#[derive(Component, Default)]
pub struct CurrentCameraZoom(f32);

impl CurrentCameraZoom {
    /// Lerp the current focus towards a target focus point.
    pub fn lerp(&mut self, target: &CameraZoom, customization: &CameraZoomCustomization) -> bool {
        if self.0.distance_squared(target.0) > 0.001 {
            self.0 = self.0.lerp(target.0, customization.smoothing);
            true
        } else {
            self.0 = target.0;
            false
        }
    }
}

#[derive(Component, Default, Copy, Clone)]
pub struct CameraZoomCustomization {
    pub smoothing: f32,
    pub standard_radius: f32,
    pub zoom_strength: f32,
    pub min: f32,
    pub max: f32,
}

// impl Default for CameraZoomCustomization {
//     fn default() -> Self {
//         Self {
//             smoothing: Vec2::new(-0.005, -0.005),
//         }
//     }
// }
