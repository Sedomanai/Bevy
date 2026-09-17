use super::info::CameraMouseInfo;
use crate::BlendProjection;
use bevy::prelude::*;
use sonolil_movement::Tracker;

#[derive(Bundle)]
pub struct TrackingCameraBundle {
    tracker: Tracker,
    transform: Transform,
    projection: Projection,
    camera: Camera,
    info: CameraMouseInfo,
}

impl TrackingCameraBundle {
    pub fn new(transform: Transform, tracker: Tracker, blend: BlendProjection) -> Self {
        Self {
            tracker,
            transform,
            projection: Projection::custom(blend),
            camera: Camera::default(),
            info: CameraMouseInfo::default(),
        }
    }
}
