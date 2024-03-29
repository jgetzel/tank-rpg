use bevy::ecs::system::{Command, EntityCommands};
use bevy::prelude::{Entity, World};
use bevy_quinnet::server::Server;
use bevy::hierarchy::Children;
use crate::simulation::events::OnObjectDespawnEvent;
use crate::simulation::server_sim::player::Player;
use crate::simulation::{Object, SyncedObjects};
use crate::simulation::Lobby;

pub trait CustomDespawnExt {
    fn custom_despawn(self);
}

struct CustomDespawn {
    pub entity: Entity,
}

impl Command for CustomDespawn {
    fn write(self, world: &mut World) {
        custom_despawn(world, self.entity);
    }
}

fn custom_despawn(world: &mut World, entity: Entity) {

    if let Some(player) = world.get::<Player>(entity) {
        let player = player.clone();
        let mut lobby = world.get_resource_mut::<Lobby>().unwrap();
        if let Some(mut data) = lobby.player_data.get_mut(&player.id) {
            data.object_id = None;
        }
    }

    if let Some(&object) = world.get::<Object>(entity) {
        let mut objects = world.get_resource_mut::<SyncedObjects>().unwrap();
        objects.objects.remove(&object.id);

        if world.get_resource::<Server>().is_some() {
            world.send_event::<OnObjectDespawnEvent>(OnObjectDespawnEvent { id: object.id });
        }
    }

    if let Some(children) = world.get::<Children>(entity) {
        let children: Vec<Entity> = children.iter().copied().collect();
        for child in children {
            custom_despawn(world, child);
        }
    }

    world.despawn(entity);
}


impl<'w, 's, 'a> CustomDespawnExt for EntityCommands<'w, 's, 'a> {
    fn custom_despawn(mut self) {
        let entity = self.id();
        self.commands().add(CustomDespawn { entity });
    }
}
