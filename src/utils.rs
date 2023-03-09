use bevy::ecs::system::EntityCommands;
use bevy::prelude::{DespawnRecursiveExt, World};
use bevy_quinnet::server::Server;
use bevy_quinnet::shared::channel::ChannelId;
use crate::networking::Lobby;
use crate::networking::messages::ServerMessage;
use crate::object::{Object, SyncedObjects};

pub trait CustomDespawn {
    fn custom_despawn(self);
}

impl<'w, 's, 'a> CustomDespawn for EntityCommands<'w, 's, 'a> {
    fn custom_despawn(mut self) {
        let entity = self.id();

        self.commands().add(move |world: &mut World| {
            let mut lobby = world.get_resource_mut::<Lobby>().unwrap();
            if let Some((&player_id, _)) = lobby.players.iter().find(|&(_, &ent)| ent == entity) {
                lobby.players.remove(&player_id);
            }

            if let Some(&object) = world.get::<Object>(entity) {
                let mut objects = world.get_resource_mut::<SyncedObjects>().unwrap();
                objects.objects.remove(&object.id);

                if let Some(server) = world.get_resource::<Server>() {
                    server.endpoint().broadcast_message_on(
                        ChannelId::UnorderedReliable,
                        ServerMessage::ObjectDespawn { object_id: object.id }
                    ).unwrap();
                }
            }
        });

        self.commands().entity(entity).despawn_recursive();
    }
}