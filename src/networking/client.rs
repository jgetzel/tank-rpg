use bevy::prelude::{Commands, Res, ResMut};
use bevy_renet::renet::{ClientAuthentication, DefaultChannel, RenetClient, RenetConnectionConfig};
use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::math::Vec2;
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use crate::assets::GameAssets;
use crate::input_helper::PlayerInput;
use crate::networking::{Lobby, PROTOCOL_ID, ServerMessages};
use crate::player::{get_player_bundle, get_turret_bundle};

pub fn new_client() -> RenetClient {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
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
    assets: Res<GameAssets>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { id } => {
                println!("Player {id} connected");
                let player_entity = commands.spawn(
                    get_player_bundle(&assets, Some(Vec2::default()))
                ).with_children(|parent| {
                    parent.spawn(get_turret_bundle(&assets));
                }).id();

                lobby.players.insert(id, player_entity);
            }
            ServerMessages::PlayerDisconnected { id } => {
                println!("Player {id} disconnected");
                if let Some(player_entity) = lobby.players.get(&id) {
                    commands.entity(*player_entity).despawn_recursive();
                }
            }
        };
    }

    // while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
    //     let players: HashMap<u64, [f32; 3]> = bincode::deserialize(&message).unwrap();
    //     for (player_id, translation) in players.iter() {
    //         if let Some(player_entity) = lobby.players.get(player_id);
    //     }
    // }
}
