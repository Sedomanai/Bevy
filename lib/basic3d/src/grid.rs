use bevy::prelude::*;
use bevy_dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings};
use sonolil_util::*;
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InfiniteGridPlugin)
            .add_systems(Startup, init.in_set(pass::StartupSpawn));
    }
}

/// Sets up the infinite grid for 3D debugging.
/// Spawns an `InfiniteGrid` entity with default settings.
fn init(mut commands: Commands) {
    commands.spawn((
        tags::WorldGrid,
        InfiniteGrid,
        InfiniteGridSettings::default(),
    ));
}
