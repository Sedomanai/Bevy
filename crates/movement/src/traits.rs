use bevy::{
    ecs::component::{Component, Mutable},
    prelude::*,
};

pub trait CopyFromGlobalTransform: Component {
    fn copy_transform(&mut self, gt: &GlobalTransform);
}

#[derive(Component, Clone, Copy)]
#[require(Transform)]
pub struct DoNotSyncOnInsert;

pub fn on_insertion_sync<Comp>(
    trigger: On<Insert, Comp>,
    mut commands: Commands,
    mut query: Query<(&mut Comp, &GlobalTransform, Has<DoNotSyncOnInsert>)>,
) where
    Comp: CopyFromGlobalTransform + Component<Mutability = Mutable>,
{
    let e = trigger.entity;
    let Ok((mut tracker, tr, no_sync)) = query.get_mut(e) else {
        return;
    };

    if no_sync {
        commands.entity(e).remove::<DoNotSyncOnInsert>();
        return;
    }

    tracker.copy_transform(tr);
}

pub fn on_copy_from_relation<Comp, Rel>(
    mut query: Query<(&mut Comp, Option<&Rel>)>,
    transforms: Query<&GlobalTransform>,
) where
    Comp: CopyFromGlobalTransform + Component<Mutability = Mutable>,
    Rel: Component + std::ops::Deref<Target = Entity>,
{
    for (mut comp, ing) in query.iter_mut() {
        let ed_global = ing.and_then(|e| transforms.get(**e).ok());

        if let Some(ed_tr) = ed_global {
            comp.copy_transform(ed_tr);
        }
    }
}
