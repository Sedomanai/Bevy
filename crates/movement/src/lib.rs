use bevy::prelude::*;

pub mod orbital;
pub mod sset;
pub mod tracker;
pub mod traits;

#[cfg(feature = "hub")]
use sonolil_hub::schedule::SonolilProcessUpdateSchedule as MovementPluginSchedule;

#[cfg(not(feature = "hub"))]
mod internal {
    pub use bevy::app::MainScheduleOrder;
    use bevy::ecs::schedule::ScheduleLabel;
    #[derive(ScheduleLabel, Default, Debug, Hash, PartialEq, Eq, Clone)]
    pub struct MovementPluginSchedule;
}

#[cfg(not(feature = "hub"))]
pub use internal::MovementPluginSchedule;

pub struct MovementPlugin;

impl Default for MovementPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(feature = "hub"))]
        {
            app.add_schedule(Schedule::new(MovementPluginSchedule));
            let mut main_schedule_order = app
                .world_mut()
                .resource_mut::<internal::MainScheduleOrder>();
            main_schedule_order.insert_before(Update, MovementPluginSchedule);
        }

        app.configure_sets(
            MovementPluginSchedule,
            sset::CopyFromRelationSet.before(sset::UpdateMovementSet),
        );
        tracker::register_systems::<MovementPluginSchedule>(app);
        orbital::register_systems::<MovementPluginSchedule>(app);
    }
}
