pub mod server;
pub mod client;
pub mod messages;
pub mod events;

use std::collections::HashMap;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_quinnet::client::Client;
use bevy_quinnet::server::Server;
use serde::{Deserialize, Serialize};
use crate::networking::messages::PlayerId;
use crate::networking::client::ClientSet::*;
use crate::networking::events::*;
use crate::networking::server::ServerSet::*;
use crate::object::ObjectId;

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

        app
            .add_event::<OnObjectDespawnEvent>()
            .add_event::<OnPlayerSpawnEvent>()
            .add_event::<OnPlayerConnectEvent>();
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerData {
    pub object_id: Option<ObjectId>,
}

impl PlayerData {
    pub fn new(object_id: ObjectId) -> Self {
        PlayerData {
            object_id: Some(object_id),
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub player_data: HashMap<PlayerId, PlayerData>,
}

impl Lobby {
    pub fn update_object_id(&mut self, player_id: PlayerId, object_id: ObjectId) -> Result<(), String> {
        if let Some(mut data) = self.player_data.get_mut(&player_id) {
            if data.object_id.is_some() {
                return Err(format!("Attempted to update object ID for Player {}, \
                but they were already connected to another object!", player_id));
            }
            data.object_id = Some(object_id);
            Ok(())
        }
        else {
            self.player_data.insert(player_id, PlayerData::new(object_id));
            Ok(())
        }
    }
}