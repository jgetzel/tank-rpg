use bevy::prelude::*;
use bevy_quinnet::client::Client;
use bevy_quinnet::server::Server;

pub mod messages;

pub fn is_client_exe(client: Option<Res<Client>>) -> bool {
    client.is_some()
}

pub fn is_client_connected(client: Option<Res<Client>>) -> bool {
    if let Some(client) = client && let Some(connection) = client.get_connection() {
        connection.is_connected()
    }
    else { false }
}

pub fn is_server_listening(server: Option<Res<Server>>) -> bool {
    if let Some(server) = server {
        server.is_listening()
    }
    else { false }
}
