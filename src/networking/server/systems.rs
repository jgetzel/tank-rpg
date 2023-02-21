use bevy::prelude::{Commands, Entity, EventReader, Query, ResMut, Transform};
use bevy_renet::renet::{DefaultChannel, RenetConnectionConfig, RenetError, RenetServer, ServerAuthentication, ServerConfig, ServerEvent};
use bevy::log::info;
use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::ErrorKind::ConnectionReset;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy_rapier2d::dynamics::Velocity;
use bevy::utils::HashMap;
use crate::assets::SpriteEnum;
use crate::networking::{Lobby, PROTOCOL_ID};
use crate::networking::client::ClientInputMessage;
use crate::networking::messages::{PhysicsObjData, ReliableMessages, UnreliableMessages};
use crate::networking::server::SERVER_ADDRESS;
use crate::object::ObjectId;
use crate::object::components::Object;
use crate::player::spawn_new_player;

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
    mut objects: Query<&Object>
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                on_client_connect(id, &mut commands, &mut server, &mut lobby, &mut objects);
            }
            ServerEvent::ClientDisconnected(id) => {
                on_client_disconnect(id, &mut commands, &mut server, &mut lobby);
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::Unreliable) {
            let message: ClientInputMessage = bincode::deserialize(&message).unwrap();

            if let Some(player_entity) = lobby.players.get(&client_id) {
                commands.entity(*player_entity).insert(message.input);
            }
        }
    }
}

pub fn server_send(
    mut server: ResMut<RenetServer>,
    query: Query<(&Object, &Transform, &Velocity, &SpriteEnum)>,
) {
    let mut objects: HashMap<ObjectId, PhysicsObjData> = HashMap::new();
    for (object, transform, vel, &sprite) in query.iter() {
        objects.insert(object.id, PhysicsObjData {
            translation: transform.translation,
            velocity: vel.linvel,
            sprite
        });
    }
    let sync_msg = bincode::serialize(&UnreliableMessages::PhysObjUpdate { objects })
        .unwrap();
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
    new_player_id: &u64,
    commands: &mut Commands,
    server: &mut RenetServer,
    lobby: &mut Lobby,
    objects: &mut Query<&Object>
) {
    info!("Player {new_player_id} Connected.");
    let new_player_entity: Entity = spawn_new_player(commands, *new_player_id, None);
    let new_object = Object::new();
    commands.entity(new_player_entity).insert(new_object);
    for (&player_id, &player) in lobby.players.iter() {
        let object_id= objects.get_mut(player).unwrap().id;
        let message = bincode::serialize(
            &ReliableMessages::PlayerConnected { player_id, object_id }
        ).unwrap();
        server.send_message(*new_player_id, DefaultChannel::Reliable, message);
    }

    lobby.players.insert(*new_player_id, new_player_entity);

    let message = bincode::serialize(
        &ReliableMessages::PlayerConnected { player_id: *new_player_id, object_id: new_object.id }
    ).unwrap();
    server.broadcast_message(DefaultChannel::Reliable, message);
}

fn on_client_disconnect(
    player_id: &u64,
    commands: &mut Commands,
    server: &mut RenetServer,
    lobby: &mut Lobby,
) {
    info!("Player {player_id} Disconnected");
    if let Some(player_entity) = lobby.players.remove(player_id) {
        commands.entity(player_entity).despawn_recursive();
    }

    let message = bincode::serialize(
        &ReliableMessages::PlayerDisconnected { player_id: *player_id }
    ).unwrap();
    server.broadcast_message(DefaultChannel::Reliable, message);
}
