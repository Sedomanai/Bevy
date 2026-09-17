use bevy::app::MainScheduleOrder;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;

// Schedule to spawn tagged entities that other crates can pool, no communication between crates allowed.
#[derive(ScheduleLabel, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnTaggedSchedule;

// Schedule to process tagged entities, communication between crates starts here.
#[derive(ScheduleLabel, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ProcessTaggedSchedule;

#[derive(ScheduleLabel, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SonolilInputSchedule;

#[derive(ScheduleLabel, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SonolilUpdateSchedule;

pub fn setup_schedule(app: &mut App) {
    app.add_schedule(Schedule::new(SpawnTaggedSchedule));
    app.add_schedule(Schedule::new(ProcessTaggedSchedule));
    app.add_schedule(Schedule::new(SonolilInputSchedule));
    app.add_schedule(Schedule::new(SonolilUpdateSchedule));

    let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();

    main_schedule_order.insert_startup_after(PreStartup, SpawnTaggedSchedule);
    main_schedule_order.insert_startup_after(SpawnTaggedSchedule, ProcessTaggedSchedule);

    main_schedule_order.insert_after(PreUpdate, SonolilInputSchedule);
    main_schedule_order.insert_after(Update, SonolilUpdateSchedule);
}
