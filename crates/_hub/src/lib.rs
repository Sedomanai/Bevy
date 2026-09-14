use bevy::prelude::*;

pub mod schedule;
pub mod tags;

pub struct Hub;

impl Plugin for Hub {
    fn build(&self, app: &mut App) {
        schedule::setup_schedule(app);
    }
}
