use bevy::prelude::*;

#[cfg(not(feature = "hub"))]
use bevy::app::Update as MovementPluginSchedule;
#[cfg(feature = "hub")]
use sonolil_hub::schedule::SonolilUpdateSchedule as MovementPluginSchedule;

use crate::{_traits::on_copy_from_relation, Orbiter, Orbiting, Tracker, Tracking};

mod _tracker_system;
use _tracker_system::update_tracker;

mod _orbital_system;
use _orbital_system::update_orbit;

pub mod system_set;
pub use system_set::{CopyFromRelationSet, UpdateMovementSet};

pub fn register_systems(app: &mut App) {
    app.add_systems(
        MovementPluginSchedule,
        on_copy_from_relation::<Orbiter, Orbiting>.in_set(CopyFromRelationSet),
    )
    .add_systems(
        MovementPluginSchedule,
        on_copy_from_relation::<Tracker, Tracking>.in_set(CopyFromRelationSet),
    )
    .add_systems(
        MovementPluginSchedule,
        update_orbit.in_set(UpdateMovementSet),
    )
    .add_systems(
        MovementPluginSchedule,
        update_tracker.in_set(UpdateMovementSet).after(update_orbit),
    );
}
