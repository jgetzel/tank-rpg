use bevy::prelude::{Commands, EventWriter, Res, ResMut};
use bevy_renet::renet::{ClientAuthentication, DefaultChannel, RenetClient, RenetConnectionConfig};
use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::hierarchy::{DespawnRecursiveExt};
use bevy::log::info;
use crate::input_helper::PlayerInput;
use crate::networking::{Lobby, PROTOCOL_ID};
use crate::networking::messages::{PhysicsObjData, ReliableMessages, UnreliableMessages};
use crate::networking::server::SERVER_ADDRESS;
use crate::object::{ObjectId};

pub struct PlayerJoinEvent {
    pub player_id: u64,
}

pub struct PlayerLeaveEvent {
    pub player_id: u64,
}

pub struct PhysObjUpdateEvent {
    pub id: ObjectId,
    pub data: PhysicsObjData
}

pub fn new_client() -> RenetClient {
    let server_addr = SERVER_ADDRESS.parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let auth = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    RenetClient::new(current_time, socket, connection_config, auth).unwrap()
}

pub fn client_send_input(
    input: Res<PlayerInput>,
    mut client: ResMut<RenetClient>,
) {
    let input_message = bincode::serialize(&*input).unwrap();
    client.send_message(DefaultChannel::Reliable, input_message);
}

pub fn client_recv(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    mut join_event: EventWriter<PlayerJoinEvent>,
    mut leave_event: EventWriter<PlayerLeaveEvent>,
    mut update_event: EventWriter<PhysObjUpdateEvent>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        let server_message: ReliableMessages = bincode::deserialize(&message).unwrap();
        match server_message {
            ReliableMessages::PlayerConnected { id } => {
                join_event.send(PlayerJoinEvent { player_id: id });
            }
            ReliableMessages::PlayerDisconnected { id } => {
                on_player_leave(id, &mut commands, &mut lobby);
                leave_event.send(PlayerLeaveEvent { player_id: id })
            }
        };
    }

    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let server_message: UnreliableMessages = bincode::deserialize(&message).unwrap();
        match server_message {
            UnreliableMessages::PhysObjUpdate { objects } => {
                for (id, data) in objects.into_iter() {
                    update_event.send( PhysObjUpdateEvent { id, data });
                }
            }
        }
    }
}

fn on_player_leave(id: u64, commands: &mut Commands, lobby: &mut Lobby) {
    info!("Player {id} Disconnected");
    if let Some(player_entity) = lobby.players.remove(&id) {
        commands.entity(player_entity).despawn_recursive();
    }
}