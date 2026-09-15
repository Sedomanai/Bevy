pub use crate::tracker::*;
use crate::*;
use bevy::ecs::schedule::ScheduleLabel;
use system_set::*;
use traits::*;

pub fn register_systems<Schedule: ScheduleLabel + Clone + Default>(app: &mut App) {
    app //.add_observer(on_insertion_sync::<Orbiter, Orbiting>)
        .add_systems(
            Schedule::default(),
            on_copy_from_relation::<Orbiter, Orbiting>.in_set(CopyFromRelationSet),
        ) //.add_observer(on_insertion_sync::<Tracker, Tracking>)
        .add_systems(
            Schedule::default(),
            on_copy_from_relation::<Tracker, Tracking>.in_set(CopyFromRelationSet),
        )
        .add_systems(Schedule::default(), update_orbit.in_set(UpdateMovementSet))
        .add_systems(
            Schedule::default(),
            update_tracker.in_set(UpdateMovementSet).after(update_orbit),
        );
}

pub fn update_orbit(mut query: Query<(&mut Transform, &Orbiter, Option<&mut Tracker>)>) {
    for (mut tr, orbiter, mut tracker) in query.iter_mut() {
        let rotation = orbiter.quaternion();
        let final_pos = orbiter.focal_point + rotation * (Vec3::Z * orbiter.radius.max(0.0));

        let tr: &mut Transform = match tracker {
            Some(ref mut tracker) => &mut tracker.transform,
            None => &mut tr,
        };

        if tr.translation != final_pos {
            tr.translation = final_pos;
        }

        if orbiter.look_at && tr.rotation != rotation {
            tr.rotation = rotation;
        }
    }
}

/// Syncs transform towards target entity (if active) or cached transform fallback.
pub fn update_tracker(
    time: Res<Time>,
    mut query: Query<(&mut Tracker, &mut Transform, Option<&Orbiter>)>,
) {
    for (tracker, tr, orbiter) in query.iter_mut() {
        if tracker.traits.0 == 0 {
            continue;
        }

        lerp_tracker_internal(tr, tracker, time.delta_secs(), orbiter);
    }
}

fn lerp_tracker_internal(
    mut tr: Mut<'_, Transform>,
    mut tracker: Mut<'_, Tracker>,
    dt: f32,
    orbiter: Option<&Orbiter>,
) {
    let track_traits = &tracker.traits;
    let local_target_tr = &tracker.transform;

    const ROTATION_THRESHOLD_0_POINT_1_DEGREES: f32 = 0.9999996; // cos(0.1 deg / 2)
    const CENTIMETER_SQURED: f32 = 0.0001;

    let translate_if = track_traits.contains(TrackerTraits::TRANSLATE)
        && tr.translation.distance_squared(local_target_tr.translation) > CENTIMETER_SQURED;
    let scale_if = track_traits.contains(TrackerTraits::SCALE)
        && tr.scale.distance_squared(local_target_tr.scale) > CENTIMETER_SQURED;
    let rotate_if = track_traits.contains(TrackerTraits::ROTATE)
        && tr.rotation.dot(local_target_tr.rotation).abs() < ROTATION_THRESHOLD_0_POINT_1_DEGREES;

    // mut here
    if tracker.snap > 0 {
        if translate_if {
            tr.translation = local_target_tr.translation;
        }
        if scale_if {
            tr.scale = local_target_tr.scale;
        }
        if rotate_if {
            tr.rotation = local_target_tr.rotation;
        }
        tracker.snap -= 1;
        return;
    }

    if !translate_if && !scale_if && !rotate_if {
        return;
    }

    let x = tracker.decay * dt;
    let factor = (x * (1.0 - x * (0.5 - x / 6.0))).clamp(0.0, 1.0);

    tr.scale = if scale_if {
        tr.scale.lerp(local_target_tr.scale, factor)
    } else {
        tr.scale
    };

    tr.translation = if translate_if {
        match orbiter {
            Some(orbiter) => {
                // Black magic

                let focal = orbiter.focal_point;
                let current_offset = tr.translation - focal;
                let current_radius = current_offset.length();
                let target_dir = (orbiter.quaternion() * Vec3::Z).normalize_or_zero();
                let current_dir = current_offset.normalize_or_zero();

                let new_dir = if current_dir == Vec3::ZERO || target_dir == Vec3::ZERO {
                    target_dir
                } else {
                    let delta = Quat::from_rotation_arc(current_dir, target_dir);
                    Quat::IDENTITY.slerp(delta, factor) * current_dir
                };

                // Smooth radius too — just a local calc from the current
                // offset length each frame, not stored anywhere.
                let target_radius = orbiter.radius.max(0.0);
                let new_radius = current_radius + (target_radius - current_radius) * factor;

                focal + new_dir * new_radius
            }
            None => tr.translation.lerp(local_target_tr.translation, factor),
        }
    } else {
        local_target_tr.translation
    };

    if rotate_if {
        match orbiter {
            Some(orbiter) => {
                if orbiter.look_at {
                    tr.look_at(orbiter.focal_point, Vec3::Y)
                }
            }
            None => tr.rotation = tr.rotation.slerp(local_target_tr.rotation, factor),
        }
    } else {
        tr.rotation = local_target_tr.rotation
    };

    // tr.rotation = if rotate_if {
    //     match orbiter {
    //         Some(orbiter) => {
    //             if orbiter.look_at {
    //                 tr.look_at(orbiter.focal_point, Vec3::Y)
    //             }
    //         }
    //         None => tr.rotation.slerp(local_target_tr.rotation, factor),
    //     }
    // } else {
    //     local_target_tr.rotation
    // };
}
