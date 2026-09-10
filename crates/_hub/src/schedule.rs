use bevy::app::MainScheduleOrder;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;

// Schedule to spawn tagged entities that other crates can pool, no communication between crates allowed.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnTaggedSchedule;

// Schedule to process tagged entities, communication between crates starts here.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ProcessTaggedSchedule;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SonolilInputSchedule;

pub fn set_schedule(app: &mut App) {
    setup_schedule(app);
}

fn setup_schedule(app: &mut App) {
    app.add_schedule(Schedule::new(SpawnTaggedSchedule));
    app.add_schedule(Schedule::new(ProcessTaggedSchedule));

    let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();

    main_schedule_order.insert_startup_after(PreStartup, SpawnTaggedSchedule);
    main_schedule_order.insert_startup_after(SpawnTaggedSchedule, ProcessTaggedSchedule);
}
