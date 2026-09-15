use bevy::{ecs::component::Component, prelude::*};

use crate::traits::*;

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
    pub struct TrackerTraits(pub u32) {
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
