use bevy::ecs::system::EntityCommands;
use bevy::log::info;
use bevy::prelude::{DespawnRecursiveExt, World};
use bevy_renet::renet::DefaultChannel::Reliable;
use bevy_renet::renet::RenetServer;
use crate::networking::Lobby;
use crate::networking::messages::ReliableMessages;
use crate::object::{Object, SyncedObjects};

pub trait CustomDespawn {
    fn custom_despawn(self);
}

impl<'w, 's, 'a> CustomDespawn for EntityCommands<'w, 's, 'a> {
    fn custom_despawn(mut self) {
        let entity = self.id();

        self.commands().add(move |world: &mut World| {
            info!("Despawn command called!");
            let mut lobby = world.get_resource_mut::<Lobby>().unwrap();
            if let Some((&player_id, _)) = lobby.players.iter().find(|&(_, &ent)| ent == entity) {
                lobby.players.remove(&player_id);
            }

            if let Some(&object) = world.get::<Object>(entity) {
                let mut objects = world.get_resource_mut::<SyncedObjects>().unwrap();
                objects.objects.remove(&object.id);

                if let Some(mut server) = world.get_resource_mut::<RenetServer>() {
                    let message = bincode::serialize(
                        &ReliableMessages::ObjectDespawn { object_id: object.id }
                    ).unwrap();
                    server.broadcast_message(Reliable, message);
                    info!("Despawn command sent!");
                }
            }
        });

        self.commands().entity(entity).despawn_recursive();
    }
}