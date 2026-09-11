use bevy::prelude::*;

#[cfg(feature = "hub")]
pub mod hub;
pub mod orbital;
pub mod sset;
pub mod tracker;
mod traits;

pub struct MovementPlugin {
    pub full_screen: bool,
    pub reactive_update_mode: bool,
    pub fps_overlay: bool,
}

impl Default for MovementPlugin {
    fn default() -> Self {
        Self {
            full_screen: false,
            reactive_update_mode: false,
            fps_overlay: false,
        }
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreUpdate,
            sset::CopyFromRelationSet.before(sset::UpdateMovementSet),
        );
        tracker::register_systems::<PreUpdate>(app);
        orbital::register_systems::<PreUpdate>(app);
    }
}
