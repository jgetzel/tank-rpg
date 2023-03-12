pub mod server;
pub mod client;
pub mod messages;

use std::collections::HashMap;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_quinnet::client::Client;
use bevy_quinnet::server::Server;
use serde::{Deserialize, Serialize};
use crate::networking::messages::PlayerId;
use crate::networking::client::ClientSet::*;
use crate::networking::server::ServerSet::*;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_set(ServerUpdate.in_base_set(CoreSet::Update))
            .configure_set(ServerReceive.before(ServerUpdate)
                .run_if(is_server_listening))
            .configure_set(ServerUpdate.before(ServerSend)
                .run_if(is_server_listening))
            .configure_set(ServerSend
                .run_if(is_server_listening))
            .configure_set(ClientReceive.before(ClientUpdate).run_if(is_client_connected))
            .configure_set(ClientUpdate.before(ClientSend).run_if(is_client_connected))
            .configure_set(ClientSend.run_if(is_client_connected));
    }
}

pub fn is_client_exe(client: Option<Res<Client>>) -> bool {
    client.is_some()
}

fn is_client_connected(client: Option<Res<Client>>) -> bool {
    if let Some(client) = client && let Some(connection) = client.get_connection() {
        connection.is_connected()
    }
    else { false }
}

fn is_server_listening(server: Option<Res<Server>>) -> bool {
    if let Some(server) = server {
        server.is_listening()
    }
    else { false }
}

// TODO Have object ID in PlayerData instead of Entity
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerData {
    pub entity: Option<Entity>,
}

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub player_data: HashMap<PlayerId, PlayerData>,
}

impl Lobby {
    pub fn get_entity(&self, id: &PlayerId) -> Option<Entity> {
        if let Some(data) = self.player_data.get(id) {
            data.entity
        } else { None }
    }

    pub fn insert_data(&mut self, id: PlayerId, data: PlayerData) -> Result<(), &'static str> {
        let old_data = self.player_data.insert(id, data);

        if let Some(PlayerData { entity }) = old_data {
            if entity.is_some() {
                return Err("Tried to insert new player data when old data still has entity assigned!");
            }
        }

        Ok(())
    }

    pub fn insert_entity(&mut self, id: PlayerId, entity: Entity) -> Result<(), &'static str> {
        let data = self.player_data.get_mut(&id);
        match data {
            Some(data) => {
                if data.entity.is_some() {
                    return Err("Player Entity already exists!");
                }
                data.entity = Some(entity);
                Ok(())
            }
            None => {
                Err("No player data to insert entity into!")
            }
        }
    }

    pub fn remove_entity(&mut self, id: &PlayerId) {
        if let Some(mut data) = self.player_data.get_mut(id) {
            data.entity = None;
        }
    }
}
