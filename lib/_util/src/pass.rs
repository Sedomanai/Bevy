use bevy::prelude::SystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct StartupSpawn;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct StartupProcess;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct StartupSubProcess;
