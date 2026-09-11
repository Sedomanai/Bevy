// use std::f32::consts::FRAC_PI_2;

use std::f32::consts::FRAC_PI_2;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

use crate::sset::*;
use crate::traits::*;

pub fn register_systems<Schedule: ScheduleLabel + Clone + Default>(app: &mut App) {
    app.add_systems(
        Schedule::default(),
        on_copy_from_relation::<Orbiter, Orbiting>.in_set(CopyFromRelationSet),
    )
    .add_systems(
        Schedule::default(),
        update_rotation.in_set(UpdateMovementSet),
    );
}

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
    radius: f32,
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

impl CopyFromGlobalTransform for Orbiter {
    fn copy_transform(&mut self, gt: &GlobalTransform) {
        self.focal_point = gt.translation();
    }
}

impl Orbiter {
    pub fn new() -> Self {
        Self { ..default() }
    }

    pub fn zoom_by_fraction(&mut self, fraction: f32) {
        self.radius -= self.radius * fraction;
    }

    pub fn zoom_by_addition(&mut self, addition: f32) {
        self.radius -= addition;
        self.radius = self.radius.max(0.0);
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

fn update_rotation(
    mut query: Query<(&mut Transform, &Orbiter, Option<&ChildOf>)>,
    transforms: Query<&GlobalTransform>,
) {
    for (mut tr, orbiter, child_of) in query.iter_mut() {
        let parent_global = child_of.and_then(|c| transforms.get(c.parent()).ok());

        let local = if let Some(parent_tr) = parent_global {
            parent_tr
                .affine()
                .inverse()
                .transform_point3(orbiter.focal_point)
        } else {
            orbiter.focal_point
        };

        let rotation = orbiter.quaternion();
        let rotate_offset = rotation * (Vec3::Z * orbiter.radius);
        let final_pos = local + rotate_offset;

        if tr.translation != final_pos {
            tr.translation = final_pos;
        }

        if orbiter.look_at && tr.rotation != rotation {
            tr.look_at(final_pos, rotation * Vec3::Y);
        }
    }
}
