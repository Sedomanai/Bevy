use bevy::{ecs::component::Mutable, prelude::*};

/// Anything that can absorb a GlobalTransform snapshot as a fallback cache.
pub trait OnRelTargetDestruction: Component {
    fn cache(&mut self, gt: &GlobalTransform);
    //fn update_each_frame(&mut self, relationship_target: &GlobalTransform, parent: &GlobalTransform);
}

/// Generic version of on_trackedby_removed / cache_orbital_before_removed.
pub fn cache_before_removed<MovementComponent, RelationTarget>(
    trigger: On<Remove, GlobalTransform>,
    mut caches: Query<&mut MovementComponent>,
    related_bys: Query<&RelationTarget>,
    transforms: Query<&GlobalTransform>,
) where
    MovementComponent: OnRelTargetDestruction + Component<Mutability = Mutable>,
    RelationTarget: Component + std::ops::Deref<Target = Vec<Entity>>,
{
    let e = trigger.entity;
    let Ok(rel) = related_bys.get(e) else { return };

    for rel in rel.iter() {
        if let (Ok(gt), Ok(mut mc)) = (transforms.get(e), caches.get_mut(*rel)) {
            mc.cache(gt);
        }
    }
}

// pub fn update_each_frame<MovementComponent, Relationship>(
//     mut query: Query<(
//         &mut MovementComponent,
//         &mut Transform,
//         Option<&Relationship>,
//         Option<&ChildOf>,
//     )>,
//     transforms: Query<&GlobalTransform>,
// ) where
//     MovementComponent: OnRelTargetDestruction + Component<Mutability = Mutable>,
//     Relationship: Component + std::ops::Deref<Target = Entity>,
// {
//     for (mut orbiter, mut tr, orbiting, child_of) in query.iter_mut() {
//         let orbited_global = orbiting.and_then(|rel| transforms.get(**rel).ok());
//         let parent_global = child_of.and_then(|c| transforms.get(c.parent()).ok());

//         // let tracked_local: Vec3 = match (orbited_global, parent_global) {
//         //     (Some(orbited), Some(parent)) => parent
//         //         .affine()
//         //         .inverse()
//         //         .transform_point3(orbited.translation()),
//         //     (Some(orbited), None) => orbited.translation(),
//         //     (None, _) => orbiter.focal_point,
//         // };
//     }
// }
