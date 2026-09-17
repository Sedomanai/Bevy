// use std::f32::consts::FRAC_PI_2;

use std::f32::consts::FRAC_PI_2;

use crate::_traits::MovementTrait;
use bevy::prelude::*;

/// Relationship of @OrbitedBy.
///
/// Unlike child/parent, the Orbiting do not inherit anything from OrbitedBy except global position as focal point.
/// Thus the Orbiter controls orbit, including whether to face the focal_point or not.
/// Their lifetimes are also decoupled, becoming easier to control outside of the child/parent relationship.
///
/// Finally, Orbiting is strictly optional; sometimes you just want to orbit around a point without a need for a focal object.
///
/// This component set is good when an Orbiter reparents its focal point a lot (like a camera.)
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Deref)]
#[relationship(relationship_target = OrbitedBy)]
pub struct Orbiting(pub Entity);

/// Relationship target of @Orbiting. See @Orbiting.
#[derive(Component, Clone, Debug, PartialEq, Eq, Deref)]
#[relationship_target(relationship = Orbiting)]
pub struct OrbitedBy(Vec<Entity>);

/// Component that orbits another point. Point may or may not be another entity (with Transform)
#[derive(Component, Copy, Clone)]
pub struct Orbiter {
    pub radius: f32,
    pub focal_point: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub look_at: bool,
}

impl Default for Orbiter {
    fn default() -> Self {
        Self {
            focal_point: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            radius: 10.0,
            look_at: false,
        }
    }
}

impl MovementTrait for Orbiter {
    fn copy_transform(&mut self, rel: &GlobalTransform, parent: Option<&GlobalTransform>) {
        self.focal_point = match parent {
            Some(pt) => pt.affine().inverse().transform_point3(rel.translation()),
            None => rel.translation(),
        };
    }
}

impl Orbiter {
    pub fn new(starting_point: Vec3, focal_point: Vec3, look_at: bool) -> Self {
        let diff = focal_point - starting_point;
        let dir = diff.normalize_or_zero();

        let yaw = (-dir.x).atan2(-dir.z);
        let pitch = dir.y.asin();
        let roll = 0.0; // Standard upright alignment

        Self {
            radius: diff.length(),
            focal_point,
            yaw,
            pitch,
            roll,
            look_at,
        }
    }

    pub fn toggle_look_at_focal_point(&mut self) {
        self.look_at = !self.look_at;
    }

    pub fn zoom(&mut self, step: i32, sensitivity: f32) {
        let s = sensitivity.max(0.0);
        self.radius *= (1.0 - s).powi(step);
    }

    pub fn set_rotation_from_quat(&mut self, quat: &Quat) {
        (self.yaw, self.pitch, self.roll) = quat.to_euler(EulerRot::YXZ);
    }

    /// Orbits based on `motion_delta` (e.g., mouse movement).
    pub fn orbit(&mut self, motion_delta: Vec2) {
        self.yaw += motion_delta.x;
        self.pitch += motion_delta.y;
        self.clamp();
    }

    // Prevent gimbal lock and keep yaw within [-PI, PI] to prevent slerp spinning issues over time.
    pub fn clamp(&mut self) {
        self.yaw = self.yaw.rem_euclid(std::f32::consts::TAU);
        if self.yaw > std::f32::consts::PI {
            self.yaw -= std::f32::consts::TAU;
        }
        let max_pitch = FRAC_PI_2 - 0.001;
        self.pitch = self.pitch.clamp(-max_pitch, max_pitch);
    }

    /// Tilt the orbital plane.
    ///
    /// Will affect yaw/pitch expression, and subsequently orbit_x and orbit_y direction.
    pub fn tilt(&mut self, delta: f32) {
        self.roll += delta;
    }

    /// Get quaternion from yaw/pitch/roll.
    pub fn quaternion(&self) -> Quat {
        Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, self.roll)
    }
}
