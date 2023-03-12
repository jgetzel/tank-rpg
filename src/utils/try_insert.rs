use bevy::ecs::system::{Command, EntityCommands};
use bevy::log::error;
use bevy::prelude::{Bundle, Entity, World};

pub trait TryInsertExt {
    fn try_insert(self, bundle: impl Bundle);
}

struct TryInsert<T> {
    pub entity: Entity,
    pub bundle: T,
}

impl<T> Command for TryInsert<T>
    where
        T: Bundle + 'static {
    fn write(self, world: &mut World) {
        if let Some(mut entity) = world.get_entity_mut(self.entity) {
            entity.insert(self.bundle);
        } else {
            error!("error[B0003]: Could not insert a bundle (of type `{}`) for entity {:?} because it doesn't exist in this World.", std::any::type_name::<T>(), self.entity);
        }
    }
}

impl<'w, 's, 'a> TryInsertExt for EntityCommands<'w, 's, 'a> {
    fn try_insert(mut self, bundle: impl Bundle) {
        let entity = self.id();
        self.commands().add(TryInsert { entity, bundle });
    }
}