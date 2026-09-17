use bevy::prelude::*;

mod _util;
use _util::_traits;

mod components;
pub use components::*;

mod systems;
pub mod system_sets {
    pub use crate::systems::{CopyFromRelationSet, UpdateMovementSet};
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        systems::register_systems(app);
    }
}
