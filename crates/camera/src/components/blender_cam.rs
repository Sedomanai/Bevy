use bevy::prelude::*;

use super::basic::TrackingCameraBundle;
use crate::BlendProjection;
use sonolil_movement::{Orbiter, Tracker};

#[derive(Debug, Component, Copy, Clone, PartialEq, Eq, Reflect)]
pub struct BlenderCamera;

#[derive(Bundle)]
pub struct BlenderCameraBundle {
    tracking: TrackingCameraBundle,
    orbiter: Orbiter,
    camera3d: Camera3d,
    blender: BlenderCamera,
}

impl BlenderCameraBundle {
    pub fn new(starting_position: Vec3, look_at_position: Vec3) -> Self {
        let transform =
            Transform::from_translation(starting_position).looking_at(look_at_position, Vec3::Y);
        let length = (look_at_position - starting_position).length();

        let tracking_bundle = TrackingCameraBundle::new(
            transform,
            Tracker::default(),
            BlendProjection::new_persp(length),
        );

        Self {
            tracking: tracking_bundle,
            orbiter: Orbiter::new(starting_position, look_at_position, true),
            camera3d: Camera3d::default(),
            blender: BlenderCamera,
        }
    }
}
