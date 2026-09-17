use bevy::prelude::*;

mod _util;
use _util::_traits;

pub mod components;

mod systems;
pub use systems::{CopyFromRelationSet, UpdateMovementSet};

pub struct MovementPlugin;

impl Default for MovementPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        systems::register_systems(app);
    }
}
