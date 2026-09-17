use bevy::prelude::*;

#[cfg(feature = "hub")]
use sonolil_hub::schedule::SonolilUpdateSchedule as MovementPluginSchedule;

#[cfg(not(feature = "hub"))]
pub use internal::MovementPluginSchedule;

#[cfg(not(feature = "hub"))]
mod internal {
    pub use bevy::app::MainScheduleOrder;
    use bevy::ecs::schedule::ScheduleLabel;

    #[derive(ScheduleLabel, Default, Debug, Hash, PartialEq, Eq, Clone)]
    pub struct MovementPluginSchedule;
}

use crate::_traits::on_copy_from_relation;
use crate::components::*;

mod _tracker_system;
use _tracker_system::*;

mod _orbital_system;
use _orbital_system::*;

pub mod system_set;
pub use system_set::*;

pub fn register_systems(app: &mut App) {
    app.add_schedule(Schedule::new(MovementPluginSchedule));
    #[cfg(not(feature = "hub"))]
    {
        let mut main_schedule_order = app
            .world_mut()
            .resource_mut::<internal::MainScheduleOrder>();
        main_schedule_order.insert_before(Update, MovementPluginSchedule);
    }

    //app.configure_schedules(schedule_build_settings)

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
