use bevy::prelude::{Children, Commands, Entity, EventReader, Query, ResMut, Transform, With};
use bevy_renet::renet::{DefaultChannel, RenetError, RenetServer, ServerEvent};
use bevy::log::info;
use std::io::ErrorKind::ConnectionReset;
use std::mem::size_of;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::tasks::{ParallelSlice, TaskPool};
use bevy_rapier2d::dynamics::Velocity;
use bevy::utils::HashMap;
use crate::asset_loader::components::SpriteEnum;
use crate::networking::Lobby;
use crate::networking::client::ClientInputMessage;
use crate::networking::messages::{PhysicsObjData, ReliableMessages, UnreliableMessages};
use crate::object::ObjectId;
use crate::object::components::Object;
use crate::player::{Player, PlayerTurret, spawn_new_player};

pub fn server_recv(
    mut server: ResMut<RenetServer>,
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut objects: Query<&Object>,
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

const UNRELIABLE_BYTE_MAX: usize = 3000;

pub fn server_send_phys_obj(
    mut server: ResMut<RenetServer>,
    query: Query<(&Object, &Transform, &Velocity, &SpriteEnum)>,
) {
    let objects: Vec<(ObjectId, PhysicsObjData)> = query.iter()
        .map(|(object, trans, vel, &sprite)| {
            (object.id, PhysicsObjData {
                translation: trans.translation,
                velocity: vel.linvel,
                sprite,
            })
        }).collect();

    let chunk_size =
        (UNRELIABLE_BYTE_MAX - 8) / (size_of::<ObjectId>() + size_of::<PhysicsObjData>());
    objects.iter()
        .par_chunk_map(&TaskPool::new(), chunk_size, |chunk| {
            chunk.iter().cloned()
                .collect::<HashMap<ObjectId, PhysicsObjData>>()
        }).into_iter().for_each(|objects|
        {
            let sync_msg = bincode::serialize(&UnreliableMessages::PhysObjUpdate { objects })
                .unwrap();
            server.broadcast_message(DefaultChannel::Unreliable, sync_msg);
        });
}

pub fn server_send_turrets(
    mut server: ResMut<RenetServer>,
    player_q: Query<(&Object, &Children), With<Player>>,
    turr_q: Query<&Transform, With<PlayerTurret>>,
) {
    let turrets = player_q.iter()
        .flat_map(|(object, children)| {
            children.iter().filter_map(|&ent| {
                match turr_q.get(ent) {
                    Ok(trans) => Some((object.id, trans.rotation)),
                    Err(_) => None,
                }
            })
        }).collect();
    let msg = bincode::serialize(&UnreliableMessages::TurretRotationUpdate { turrets })
        .unwrap();
    server.broadcast_message(DefaultChannel::Unreliable, msg)
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
    objects: &mut Query<&Object>,
) {
    info!("Player {new_player_id} Connected.");
    let new_player_entity: Entity = spawn_new_player(commands, *new_player_id, None);
    let new_object = Object::new();
    commands.entity(new_player_entity).insert(new_object);
    for (&player_id, &player) in lobby.players.iter() {
        let object_id = objects.get_mut(player).unwrap().id;
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
