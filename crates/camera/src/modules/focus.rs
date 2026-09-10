use bevy::prelude::*;

/// Desired pint in 3D space that the camera should focus on, which can be a static `Vec3` or an `Entity`.
#[derive(Component, Default, Copy, Clone)]
pub struct SonolilCamera {
    focal_point: Vec3,
    target: Option<Entity>,
    witness: Option<Entity>,
}
