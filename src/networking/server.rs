use std::default::Default;
use bevy_renet::renet::{DefaultChannel, RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig, ServerEvent};
use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::{DespawnRecursiveExt, Entity, EventReader, Res, ResMut};
use crate::assets::GameAssets;
use crate::networking::{Lobby, PROTOCOL_ID, ServerMessages};
use crate::player::spawn_new_player;

pub fn new_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(
        64,
        PROTOCOL_ID,
        server_addr,
        ServerAuthentication::Unsecure);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

pub fn server_recv(
    mut server: ResMut<RenetServer>,
    mut server_events: EventReader<ServerEvent>,
    mut commands: bevy::prelude::Commands,
    assets: Res<GameAssets>,
    mut lobby: ResMut<Lobby>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                println!("Player {id} connected.");
                let player_entity: Entity = spawn_new_player(&mut commands, &assets, None);

                for &player_id in lobby.players.keys() {
                    let message = bincode::serialize(
                        &ServerMessages::PlayerConnected { id: player_id }
                    ).unwrap();
                    server.send_message(*id, DefaultChannel::Reliable, message);
                }

                lobby.players.insert(*id, player_entity);

                let message = bincode::serialize(
                    &ServerMessages::PlayerConnected { id: *id }
                ).unwrap();
                server.broadcast_message(DefaultChannel::Reliable, message);
            },
            ServerEvent::ClientDisconnected(id) => {
                println!("Player {id} Disconnected");
                if let Some(player_entity) = lobby.players.remove(id) {
                    commands.entity(player_entity).despawn_recursive();
                }

                let message = bincode::serialize(
                    &ServerMessages::PlayerDisconnected {id: *id}
                ).unwrap();
                server.broadcast_message(DefaultChannel::Reliable, message);
            }
        }
    }
}