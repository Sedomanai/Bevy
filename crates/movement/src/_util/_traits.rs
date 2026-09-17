use bevy::{
    ecs::component::{Component, Mutable},
    prelude::*,
};

pub trait MovementTrait: Component {
    //fn sync_on_insert(&mut self, gt: &GlobalTransform);
    fn copy_transform(&mut self, gt: &GlobalTransform, pt: Option<&GlobalTransform>);
}

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
