use bevy::prelude::SystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CopyFromRelationSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UpdateMovementSet;
