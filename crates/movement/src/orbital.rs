// use std::f32::consts::FRAC_PI_2;

use bevy::{
    ecs::schedule::ScheduleLabel,
    math::{NormedVectorSpace, VectorSpace},
    prelude::*,
};

use crate::traits::*;

pub fn register_systems<S: ScheduleLabel + Clone + Default>(app: &mut App) {
    app.add_observer(cache_before_removed::<Orbiter, OrbitedBy>)
        .add_systems(S::default(), update_rotation);
}

// Unlike Trackers, I've debated myself whether Orbiter needs a custom relationship.
// A child is already designed to orbit around the parent, after all.
//
// After much consideration, I've decided that decoupling despawn lifetime is still worth it.
// With parent-child, the child's lifetime is hard to control outside of the parent's.
//
// Also, in the parent-child's case, the parent controls the orbit, whereas here, the Orbiter does.
// The rotation of the Orbiting is decoupled from OrbitedBy's rotation; only the position is tracked.
// Besides, sometimes you just want to orbit around a point without a need for a parent object / relationship.
//
// In short, this component set is good when an Orbiter reparents its focal point a lot (like a camera.)

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
#[relationship(relationship_target = OrbitedBy)]
pub struct Orbiting(pub Entity);

#[derive(Component, Clone, Debug, PartialEq, Eq, Deref)]
#[relationship_target(relationship = Orbiting)]
pub struct OrbitedBy(Vec<Entity>);

#[derive(Component, Copy, Clone)]
pub struct Orbiter {
    focal_point: Vec3,
    yaw: f32,
    pitch: f32,
    roll: f32,
    radius: f32,
}

impl Default for Orbiter {
    fn default() -> Self {
        Self {
            focal_point: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            radius: 10.0,
        }
    }
}

impl Orbiter {
    pub fn new() -> Self {
        Self { ..default() }
    }
}

impl OnRelTargetDestruction for Orbiter {
    fn cache(&mut self, gt: &GlobalTransform) {
        self.focal_point = gt.translation();
    }
}

fn update_rotation(
    mut query: Query<(
        &mut Transform,
        &Orbiter,
        Option<&Orbiting>,
        Option<&ChildOf>,
    )>,
    transforms: Query<&GlobalTransform>,
) {
    for (mut tr, orbiter, orbiting, child_of) in query.iter_mut() {
        let orbited_global = orbiting.and_then(|e| transforms.get(e.0).ok());
        let parent_global = child_of.and_then(|c| transforms.get(c.parent()).ok());

        let local_orbit_point: Vec3 = match (orbited_global, parent_global) {
            (Some(orbited), Some(parent)) => parent
                .affine()
                .inverse()
                .transform_point3(orbited.translation()),
            (Some(orbited), None) => orbited.translation(),
            (None, _) => orbiter.focal_point,
        };

        let rotation = Quat::from_euler(EulerRot::YXZ, orbiter.yaw, orbiter.pitch, orbiter.roll);
        let rotate_offset = rotation * (Vec3::Z * orbiter.radius);

        let final_pos = local_orbit_point + rotate_offset;

        if tr.translation != final_pos {
            tr.translation = final_pos;
        }
        if tr.rotation != rotation {
            tr.rotation = rotation;
        }
    }
}

// fn zoom(parent_tr: &Transform, tr: &mut Transform, delta: f32) {}

// use crate::*;

// /// Target Orbit. Standard yaw, pitch, and roll.
// ///
// /// Modularized to work with both 1st and 3rd-person orbits (depending on the existence of Focus).
// #[derive(Component, Default, Copy, Clone)]
// pub struct TargetOrbit {
//     /// The target yaw around Focus point, if none then around itself.
//     pub yaw: f32,
//     /// The target pitch around Focus point, if none then around itself.
//     pub pitch: f32,
//     /// The target roll that tilts the final orbit.
//     pub roll: f32,
// }

// impl TargetOrbit {
//     pub fn set(&mut self, quat: &Quat) {
//         (self.yaw, self.pitch, self.roll) = quat.to_euler(EulerRot::YXZ);
//     }

//     /// Orbits based on `motion_delta` (e.g., mouse movement).
//     pub fn orbit(&mut self, motion_delta: Vec2) {
//         self.yaw += motion_delta.x;
//         self.pitch += motion_delta.y;
//     }

//     // Prevent gimbal lock and keep yaw within [-PI, PI] to prevent slerp spinning issues over time.
//     pub fn clamp(&mut self) {
//         self.yaw = self.yaw.rem_euclid(std::f32::consts::TAU);
//         if self.yaw > std::f32::consts::PI {
//             self.yaw -= std::f32::consts::TAU;
//         }
//         let max_pitch = FRAC_PI_2 - 0.001;
//         self.pitch = self.pitch.clamp(-max_pitch, max_pitch);
//     }

//     /// Tilt the orbital plane.
//     ///
//     /// Will affect yaw/pitch expression, and subsequently orbit_x and orbit_y direction.
//     pub fn tilt(&mut self, delta: f32) {
//         self.roll += delta;
//     }

//     /// Update tick to interpolate Transform. Basically ticks for every TargetOrbit in the world.
//     ///
//     /// In case you don't want this for a specific component, write your own systems or consider [`snap`](Self::snap).
//     pub fn tick(&self, mc: &mut TargetTransform, delta_time: f32) {
//         let tr = &mut mc.transform;
//         let target_rotation = tr.rotation.slerp(self.quaternion(), mc.decay * delta_time);
//         tr.rotation = target_rotation;
//     }

//     /// Snap transform to this orbit.
//     pub fn snap(&self, mc: &mut TargetTransform) {
//         mc.transform.rotation = self.quaternion();
//     }

//     /// Get quaternion from yaw/pitch/roll.
//     pub fn quaternion(&self) -> Quat {
//         Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, self.roll)
//     }

//     /// See if two Quats are the same rotation.
//     ///
//     /// TODO: move this to math function somewhere in the future
//     fn is_same_rotation(&self, v1: &Quat, v2: &Quat) -> bool {
//         v1.dot(*v2).abs() > 0.9999
//     }
// }

// // /// Current Orbit. Standard yaw, pitch, and roll.
// // ///
// // /// Modularized to work with both 1st and 3rd-person orbits (depending on the existence of Focus).
// // #[derive(Component, Default, Copy, Clone)]
// // #[require(TargetOrbit)]
// // pub struct CurrentOrbit {
// //     /// The current yaw around Focus point, if none then around itself.
// //     pub yaw: f32,
// //     /// The current pitch around Focus point, if none then around itself.
// //     pub pitch: f32,
// //     /// The current roll that tilts the final orbit.
// //     pub roll: f32,
// // }

// // impl CurrentOrbit {
// //     /// Update tick to interpolate Orbit.
// //     ///
// //     /// Ticks for every CurrentOrbit in the world.
// //     pub fn tick(&mut self, target: &TargetOrbit, mc: &mut MovementComponent, delta_time: f32) {
// //         self.yaw = self.yaw.lerp(target.yaw, factor);
// //         self.pitch = self.pitch.lerp(target.pitch, factor);
// //         self.roll = self.roll.lerp(target.roll, factor);

// //         let tr = &mut mc.target_transform;
// //         let factor = mc.smooth_value * delta_time;
// //         let target_rotation = tr.rotation.slerp(self.quaternion(), factor);
// //         tr.rotation = target_rotation;
// //     }

// //     /// Get quaternion from yaw/pitch/roll.
// //     pub fn quaternion(&self) -> Quat {
// //         Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, self.roll)
// //     }
// // }

// // TODO: make it available when necessary
// //
// // /// Incremental orbit on update. For automatic orbits based on speed or
// // ///
// // /// Modularized to work with both 1st and 3rd-person orbits (depending on the existence of Focus).
// // pub struct DeltaOrbit {
// //     /// The yaw around Focus point, if none then around itself.
// //     pub yaw: f32,
// //     /// The pitch around Focus point, if none then around itself.
// //     pub pitch: f32,
// //     /// The roll that tilts the final orbit.
// //     pub roll: f32,
// // }

// /// Orbital Focus.
// #[derive(Component, Copy, Clone)]
// #[require(TargetOrbit, CurrentFocus)]
// #[component(on_remove = on_remove_focus)]
// pub struct TargetFocus {
//     pub point: Vec3,
//     pub radius: f32,
// }

// impl Default for TargetFocus {
//     fn default() -> Self {
//         Self {
//             point: Vec3::ZERO,
//             radius: 10.0,
//         }
//     }
// }

// impl TargetFocus {
//     /// Zoom based on `fraction` (e.g., mouse scroll). Value not clamped.
//     pub fn zoom(&mut self, delta: f32) {
//         self.radius -= delta * self.radius;
//     }
// }

// /// Current point in 3D space that the camera is focused on.
// ///
// /// Discoupled from Radius to allow accuate Changed flags.
// #[derive(Component, Default, Clone, Copy)]
// #[require(TargetOrbit, TargetFocus)]
// #[component(on_insert = on_insert_focus)]
// #[component(on_remove = on_remove_focus)]
// pub struct CurrentFocus {
//     pub point: Vec3,
//     pub radius: f32,
// }

// impl CurrentFocus {
//     /// Update tick to interpolate Focus. Ticks for every (Current)Focus in the world.
//     ///
//     /// Factor not clamped to 0..1, beware.
//     ///
//     /// If you want to snap, copy Focus to CurrentFocus.
//     pub fn tick(&mut self, target: &TargetFocus, factor: f32) {
//         self.point = self.point.lerp(target.point, factor);
//         self.radius = self.radius.lerp(target.radius, factor);
//     }
// }

// fn on_insert_focus(mut world: DeferredWorld, context: HookContext) {
//     // 1. Read the current focus data directly from DeferredWorld
//     let focus = world.get::<TargetFocus>(context.entity).copied();

//     if let Some(focus) = focus {
//         // 2. Queue a command to update/insert TargetFocus with the synced value
//         world
//             .commands()
//             .entity(context.entity)
//             .insert(CurrentFocus {
//                 point: focus.point,
//                 radius: focus.radius,
//             });
//     }
// }

// fn on_remove_focus(mut world: DeferredWorld, context: HookContext) {
//     world
//         .commands()
//         .entity(context.entity)
//         .remove::<CurrentFocus>()
//         .remove::<TargetFocus>();
// }

// /// Sync Targets
// ///
// /// If target entity exists, sync focus to its position.
// ///
// /// If both target and follow entity exist, sync radius to their distance.
// pub fn sync_targets(
//     time: Res<Time>,
//     mut query: Query<
//         (
//             &mut CurrentFocus,
//             &mut TargetFocus,
//             Option<&crate::TargetEntity>,
//             Option<&crate::FollowEntity>,
//         ),
//         With<TargetTransform>,
//     >,
//     transforms: Query<&Transform>,
// ) {
//     let dt = time.delta_secs();

//     for (mut curr, mut dest, target, follow) in query.iter_mut() {
//         let target = target.and_then(|e| transforms.get(e.0).ok());
//         let follow = follow.and_then(|e| transforms.get(e.0).ok());

//         if let Some(target) = target {
//             dest.point = target.translation;
//         }
//         if let Some(follow) = follow {
//             curr.0.radius = dest.point.distance(follow.translation);
//         }

//         curr.tick(&dest, dest.factor_multiplier * dt);
//     }
// }

// pub fn sync_orbits(
//     mut query: Query<
//         (
//             &mut CurrentFocus,
//             &mut TargetFocus,
//             Option<&crate::TargetEntity>,
//             Option<&crate::FollowEntity>,
//         ),
//         With<TargetTransform>,
//     >,
// ) {
// }

// // pub fn sync_orbits(
// //     time: Res<Time>,
// //     query: Query<(
// //         &mut TargetOrbit,
// //         Option<&crate::TargetEntity>,
// //         Option<&crate::FollowEntity>,
// //     )>,
// //     transforms: Query<&Transform>,
// // ) {
// //     let dt = time.delta_secs();
// //     for (mut dest, target, follow) in query.iter_mut() {
// //         // To avoid nesting..
// //         let Some(target) = target else {
// //             curr.tick(&dest, dt);
// //             continue;
// //         };
// //         let Ok(target) = transforms.get(target.0) else {
// //             curr.tick(&dest, dt);
// //             continue;
// //         };
// //         dest.point = target.translation;

// //         if let Some(follow) = follow {
// //             if let Ok(follow) = transforms.get(follow.0) {
// //                 curr.0.radius = target.translation.distance(follow.translation);
// //             }
// //         }

// //         curr.tick(&dest, dt);
// //     }
// // }

// // pub fn orbital_system(
// //     time: Res<Time>,
// //     mut sync_targets: ParamSet<(
// //         Query<(
// //             &mut CurrentFocus,
// //             &TargetFocus,
// //             Option<&crate::TargetEntity>,
// //             Option<&crate::FollowEntity>,
// //         )>,
// //     )>,
// //     mut sync_orbits: ParamSet<(
// //         Query<(&mut MovementComponent, &TargetOrbit), Without<crate::FollowEntity>>,
// //         Query<(&mut MovementComponent, &TargetOrbit, &FollowEntity)>,
// //     )>,
// //     mut final_polish: ParamSet<(
// //         Query<
// //             (&mut MovementComponent, &TargetOrbit, &CurrentFocus),
// //             Without<crate::FollowEntity>,
// //         >,
// //         Query<(&mut MovementComponent, &TargetOrbit, &CurrentFocus), With<crate::FollowEntity>>,
// //     )>,
// //     transforms: Query<&Transform>,
// // ) {
// //     let dt = time.delta_secs();
// // }

// // /////// Sync Orbits
// // for (target, mut ticker) in sync_orbits.p0().iter_mut() {
// //     ticker.tick(target, dt);
// // }

// // for (mut mc, mut curr, target) in ticks.p1().iter_mut() {
// //     curr.tick(target, dt);
// // }

// // for (mc, orbit, focus) in focus_tick.p1().iter_mut() {
// //     let focus_point = focus.0.point;
// //     let radius = focus.0.radius;
// //     let rotation = orbit.quaternion();

// //     let target = focus_point + (mc.target_transform.rotation * Vec3::Z) * radius;

// //     tr.translation = position;
// //     let current = tr.bypass_change_detection().translation;
// //     let position = current.lerp(target, mc.smooth_value * dt);

// //     if current.distance_squared(position) > 0.00000001 {
// //     } else if target != current {
// //         tr.bypass_change_detection().translation = target;
// //     }
// // }
