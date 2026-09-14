use bevy::{
    ecs::component::{Component, Mutable},
    prelude::*,
};

pub trait MovementTrait: Component {
    //fn sync_on_insert(&mut self, gt: &GlobalTransform);
    fn copy_transform(&mut self, gt: &GlobalTransform, pt: Option<&GlobalTransform>);
}

// #[derive(Component, Clone, Copy)]
// #[require(Transform)]
// pub struct DoNotSyncOnInsert;

// pub fn on_insertion_sync<Comp, Rel>(
//     trigger: On<Insert, Comp>,
//     mut commands: Commands,
//     mut query: Query<(
//         &mut Comp,
//         &GlobalTransform,
//         Option<&Rel>,
//         Option<&ChildOf>,
//         Has<DoNotSyncOnInsert>,
//     )>,
//     transforms: Query<&GlobalTransform>,
// ) where
//     Comp: MovementTrait + Component<Mutability = Mutable>,
//     Rel: Component + std::ops::Deref<Target = Entity>,
// {
//     let e = trigger.entity;
//     let Ok((mut er, tr, ing, child_of, no_sync)) = query.get_mut(e) else {
//         return;
//     };

//     if no_sync {
//         commands.entity(e).remove::<DoNotSyncOnInsert>();
//         return;
//     }

//     let ed = ing.and_then(|e| transforms.get(**e).ok());

//     match ed {
//         Some(ed) => {
//             let pt = child_of.and_then(|c| transforms.get(c.parent()).ok());
//             er.copy_transform(ed, pt);
//         }
//         None => er.sync_on_insert(tr),
//     }
// }

pub fn on_copy_from_relation<Comp, Rel>(
    mut query: Query<(&mut Comp, Option<&Rel>, Option<&ChildOf>)>,
    transforms: Query<&GlobalTransform>,
) where
    Comp: MovementTrait + Component<Mutability = Mutable>,
    Rel: Component + std::ops::Deref<Target = Entity>,
{
    for (mut comp, ing, child_of) in query.iter_mut() {
        let ed_global = ing.and_then(|e| transforms.get(**e).ok());
        let parent_global = child_of.and_then(|c| transforms.get(c.parent()).ok());

        if let Some(ed_tr) = ed_global {
            comp.copy_transform(ed_tr, parent_global);
        }
    }
}
