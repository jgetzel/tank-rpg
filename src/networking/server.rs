use std::default::Default;
use std::io::ErrorKind::ConnectionReset;
use bevy_renet::renet::{DefaultChannel, RenetConnectionConfig, RenetError, RenetServer, ServerAuthentication, ServerConfig, ServerEvent};
use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, EventReader, Query, ResMut, Transform};
use bevy::utils::HashMap;
use crate::input_helper::PlayerInput;
use crate::networking::{Lobby, PROTOCOL_ID, ServerMessages};
use crate::player::{Player, spawn_new_player};

pub const SERVER_ADDRESS: &str = "127.0.0.1:5000";

pub fn new_server() -> RenetServer {
    let server_addr = SERVER_ADDRESS.parse().unwrap();
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
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                on_client_connect(id, &mut commands, &mut server, &mut lobby);
            }
            ServerEvent::ClientDisconnected(id) => {
                on_client_disconnect(id, &mut commands, &mut server, &mut lobby);
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::Reliable) {
            let player_input: PlayerInput = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                commands.entity(*player_entity).insert(player_input);
            }
        }
    }
}

pub fn broadcast_state(
    mut server: ResMut<RenetServer>,
    query: Query<(&Player, &Transform)>,
) {
    let mut players: HashMap<u64, [f32; 3]> = HashMap::new();
    for (player, transform) in query.iter() {
        players.insert(player.id, transform.translation.into());
    }
    let sync_msg = bincode::serialize(&players).unwrap();
    server.broadcast_message(DefaultChannel::Unreliable, sync_msg);
}

pub fn force_disconnect_handler(mut renet_err: EventReader<RenetError>) {
    for e in renet_err.iter() {
        if let RenetError::IO(e) = e {
            if e.kind() == ConnectionReset {
                return;
            }
        };

        panic!("{e:?}");
    }
}

fn on_client_connect(
    id: &u64,
    commands: &mut Commands,
    server: &mut RenetServer,
    lobby: &mut Lobby,
) {
    println!("Player {id} Connected.");
    let player_entity: Entity = spawn_new_player(commands, *id, None);

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
}

fn on_client_disconnect(
    id: &u64,
    commands: &mut Commands,
    server: &mut RenetServer,
    lobby: &mut Lobby,
) {
    println!("Player {id} Disconnected");
    if let Some(player_entity) = lobby.players.remove(id) {
        commands.entity(player_entity).despawn_recursive();
    }

    let message = bincode::serialize(
        &ServerMessages::PlayerDisconnected { id: *id }
    ).unwrap();
    server.broadcast_message(DefaultChannel::Reliable, message);
}