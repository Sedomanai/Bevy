use bevy::{
    ecs::{component::Component, schedule::ScheduleLabel},
    prelude::*,
};

use crate::traits::*;

pub fn register_systems<S: ScheduleLabel + Clone + Default>(app: &mut App) {
    app.add_observer(on_tracker_inserted)
        .add_observer(cache_before_removed::<TrackerTransform, TrackedBy>)
        .add_systems(S::default(), lerp_target_transform);
}

// The reason Tracker needs custom relationship machinery is three specific things:
//
// Smoothing (lerp instead of rigid snap), hard with parent-child
// Cross-hierarchy targets (tracking something you're not a child of), and
// Decoupled despawn lifetime.

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
#[require(TrackerTransform)]
#[relationship(relationship_target = TrackedBy)]
pub struct Tracking(pub Entity);

#[derive(Component, Clone, Debug, PartialEq, Eq, Deref)]
#[relationship_target(relationship = Tracking)]
pub struct TrackedBy(Vec<Entity>);

/// Component holding interpolation state and cached target transform.
#[derive(Component)]
#[require(Transform)]
pub struct TrackerTransform {
    pub decay: f32,
    pub transform: Transform,
    pub snap: bool,
    pub active: bool,
}

impl Default for TrackerTransform {
    fn default() -> Self {
        Self {
            decay: 10.0,
            transform: Transform::IDENTITY,
            snap: true,
            active: true,
        }
    }
}

impl OnRelTargetDestruction for TrackerTransform {
    fn cache(&mut self, gt: &GlobalTransform) {
        self.transform = gt.compute_transform();
    }
}

impl TrackerTransform {
    pub fn new(decay: f32) -> Self {
        Self { decay, ..default() }
    }
}

#[derive(Component, Clone, Copy)]
#[require(Transform)]
pub struct TrackerDoNotSyncOnInsert;

fn on_tracker_inserted(
    trigger: On<Insert, TrackerTransform>,
    mut commands: Commands,
    mut query: Query<(
        &mut TrackerTransform,
        &GlobalTransform,
        Has<TrackerDoNotSyncOnInsert>,
    )>,
) {
    let e = trigger.entity;
    let Ok((mut tracker, tr, no_sync)) = query.get_mut(e) else {
        return;
    };

    if no_sync {
        commands.entity(e).remove::<TrackerDoNotSyncOnInsert>();
        return;
    }

    tracker.transform = tr.compute_transform();
}

/// Syncs transform towards target entity (if active) or cached transform fallback.
fn lerp_target_transform(
    time: Res<Time>,
    mut query: Query<(
        &mut TrackerTransform,
        &mut Transform,
        Option<&Tracking>,
        Option<&ChildOf>,
    )>,
    transforms: Query<&GlobalTransform>,
) {
    for (mut tracker, mut tr, tracking, child_of) in query.iter_mut() {
        if !tracker.active {
            continue;
        }

        let tracked_global = tracking.and_then(|e| transforms.get(e.0).ok());
        let parent_global = child_of.and_then(|c| transforms.get(c.parent()).ok());

        let tracked_local = match (tracked_global, parent_global) {
            (Some(tracked), Some(parent)) => tracked.reparented_to(parent),
            (Some(tracked), None) => tracked.compute_transform(),
            (None, _) => tracker.transform,
        };

        lerp_to_target_internal(
            &mut tr,
            &tracked_local,
            tracker.decay * time.delta_secs(),
            tracker.snap,
        );

        if tracker.snap {
            tracker.snap = false;
        }
    }
}

fn lerp_to_target_internal(tr: &mut Transform, target_tr: &Transform, x: f32, snap: bool) {
    if tr == target_tr {
        return;
    }

    if snap {
        *tr = *target_tr;
        return;
    }

    let translate_if = tr.translation.distance_squared(target_tr.translation) > 0.0001;
    let scale_if = tr.scale.distance_squared(target_tr.scale) > 0.0001;
    let rotate_if = tr.rotation.dot(target_tr.rotation).abs() < 0.9999;

    let factor = (x * (1.0 - x * (0.5 - x / 6.0))).clamp(0.0, 1.0);

    tr.translation = if translate_if {
        tr.translation.lerp(target_tr.translation, factor)
    } else {
        target_tr.translation
    };

    tr.rotation = if rotate_if {
        tr.rotation.slerp(target_tr.rotation, factor)
    } else {
        target_tr.rotation
    };

    tr.scale = if scale_if {
        tr.scale.lerp(target_tr.scale, factor)
    } else {
        tr.scale
    };
}
