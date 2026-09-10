use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct CameraOrbit {
    pub sensitivity: Vec2,
    pub max_pitch: f32,
    pub min_pitch: f32,
}

impl Default for CameraOrbit {
    fn default() -> Self {
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;
        Self {
            sensitivity: Vec2::new(-0.005, -0.005),
            max_pitch,
            min_pitch: -max_pitch,
        }
    }
}
