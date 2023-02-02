use bevy::prelude::{Commands, EventWriter, Res, ResMut, Transform, Vec3};
use bevy_renet::renet::{ClientAuthentication, DefaultChannel, RenetClient, RenetConnectionConfig};
use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::math::Vec2;
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::utils::{default, HashMap};
use crate::input_helper::PlayerInput;
use crate::networking::{Lobby, PROTOCOL_ID, ServerMessages};
use crate::networking::server::SERVER_ADDRESS;
use crate::player::{get_player_bundle, get_turret_bundle, Player};

pub struct PlayerJoinEvent {
    pub player_id: u64,
}

pub struct PlayerLeaveEvent {
    pub player_id: u64,
}

pub struct PlayerUpdateEvent {
    pub player_id: u64,
    pub translation: Vec3,
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
    mut update_event: EventWriter<PlayerUpdateEvent>
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { id } => {
                on_new_player(id, &mut commands, &mut lobby);
                join_event.send(PlayerJoinEvent { player_id: id });
            }
            ServerMessages::PlayerDisconnected { id } => {
                on_player_leave(id, &mut commands, &mut lobby);
                leave_event.send(PlayerLeaveEvent { player_id: id })
            }
        };
    }

    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let players: HashMap<u64, [f32; 3]> = bincode::deserialize(&message).unwrap();
        for (&player_id, &translation) in players.iter() {
            let translation = Vec3::from(translation);
            update_event.send( PlayerUpdateEvent { player_id, translation });
        }
    }
}

fn on_new_player(id: u64, commands: &mut Commands, lobby: &mut Lobby) {
    println!("Player {id} Connected");
    let player_entity = commands.spawn(
        get_player_bundle(id, Some(Vec2::default()))
    ).with_children(|parent| {
        parent.spawn(get_turret_bundle());
    }).id();

    lobby.players.insert(id, player_entity);
}

fn on_player_leave(id: u64, commands: &mut Commands, lobby: &mut Lobby) {
    println!("Player {id} Disconnected");
    if let Some(player_entity) = lobby.players.remove(&id) {
        commands.entity(player_entity).despawn_recursive();
    }
}