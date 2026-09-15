use bevy::prelude::*;

pub mod orbital;
use orbital::*;

pub mod system_set;
pub mod tracker;

mod systems;
mod traits;

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

pub use crate::tracker::*;

pub struct MovementPlugin;

impl Default for MovementPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_schedule(Schedule::new(MovementPluginSchedule));
        #[cfg(not(feature = "hub"))]
        {
            let mut main_schedule_order = app
                .world_mut()
                .resource_mut::<internal::MainScheduleOrder>();
            main_schedule_order.insert_before(Update, MovementPluginSchedule);
        }

        app.configure_sets(
            MovementPluginSchedule,
            system_set::CopyFromRelationSet.before(system_set::UpdateMovementSet),
        );

        systems::register_systems::<MovementPluginSchedule>(app);
    }
}
