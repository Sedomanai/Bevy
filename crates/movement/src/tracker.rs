use bevy::{
    ecs::{component::Component, schedule::ScheduleLabel},
    prelude::*,
};

use crate::sset::*;
use crate::traits::*;

pub fn register_systems<S: ScheduleLabel + Clone + Default>(app: &mut App) {
    app //.add_observer(on_insertion_sync::<Tracker, Tracking>)
        .add_systems(
            S::default(),
            on_copy_from_relation::<Tracker, Tracking>.in_set(CopyFromRelationSet),
        )
        .add_systems(S::default(), lerp_tracker.in_set(UpdateMovementSet));
}

// The reason Tracker needs custom relationship machinery is three specific things:
//
// Smoothing (lerp instead of rigid snap), hard with parent-child
// Cross-hierarchy targets (tracking something you're not a child of), and
// Decoupled despawn lifetime.
#[derive(Component, Clone, Copy, Deref, Debug, PartialEq, Eq)]
#[require(Tracker)]
#[relationship(relationship_target = TrackedBy)]
pub struct Tracking(pub Entity);

#[derive(Component, Clone, Debug, PartialEq, Eq, Deref)]
#[relationship_target(relationship = Tracking)]
pub struct TrackedBy(Vec<Entity>);

sonolil_util::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct TrackerTraits(u32) {
        const TRANSLATE = 0b0001;
        const ROTATE    = 0b0010;
        const SCALE     = 0b0100; // Not default
    }
}

/// Component holding interpolation state and cached target transform.
#[derive(Component)]
#[require(Transform)]
pub struct Tracker {
    pub decay: f32,
    pub transform: Transform,
    pub traits: TrackerTraits,
    pub snap: u32,
}

impl Default for Tracker {
    fn default() -> Self {
        Self {
            decay: 10.0,
            transform: Transform::IDENTITY,
            traits: TrackerTraits::TRANSLATE | TrackerTraits::ROTATE,
            snap: 2,
        }
    }
}

impl Tracker {
    pub fn new(decay: f32, traits: TrackerTraits) -> Self {
        Self {
            decay,
            traits,
            ..default()
        }
    }
}

impl MovementTrait for Tracker {
    fn copy_transform(&mut self, rel: &GlobalTransform, pt: Option<&GlobalTransform>) {
        self.transform = match pt {
            Some(pt) => rel.reparented_to(pt),
            None => rel.compute_transform(),
        };
    }
}

/// Syncs transform towards target entity (if active) or cached transform fallback.
fn lerp_tracker(time: Res<Time>, mut query: Query<(&mut Tracker, &mut Transform)>) {
    for (tracker, tr) in query.iter_mut() {
        if tracker.traits.0 == 0 {
            continue;
        }
        lerp_tracker_internal(tr, tracker, time.delta_secs());
    }
}

fn lerp_tracker_internal(mut tr: Mut<'_, Transform>, mut tracker: Mut<'_, Tracker>, dt: f32) {
    let track_traits = &tracker.traits;
    let local_target_tr = &tracker.transform;

    let translate_if = track_traits.contains(TrackerTraits::TRANSLATE)
        && tr.translation.distance_squared(local_target_tr.translation) > 0.0001;
    let scale_if = track_traits.contains(TrackerTraits::SCALE)
        && tr.scale.distance_squared(local_target_tr.scale) > 0.0001;
    let rotate_if = track_traits.contains(TrackerTraits::ROTATE)
        && tr.rotation.dot(local_target_tr.rotation).abs() < 0.9999;

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

    tr.translation = if translate_if {
        tr.translation.lerp(local_target_tr.translation, factor)
    } else {
        local_target_tr.translation
    };

    tr.rotation = if rotate_if {
        tr.rotation.slerp(local_target_tr.rotation, factor)
    } else {
        local_target_tr.rotation
    };

    tr.scale = if scale_if {
        tr.scale.lerp(local_target_tr.scale, factor)
    } else {
        tr.scale
    };
}
