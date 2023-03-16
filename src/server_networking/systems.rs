use bevy::prelude::{Children, Commands, EventReader, EventWriter, GlobalTransform, NextState, Query, Res, ResMut, Transform, With};
use bevy::log::info;
use std::mem::size_of;
use std::net::Ipv4Addr;
use bevy::tasks::{ParallelSlice, TaskPool};
use bevy_rapier2d::dynamics::Velocity;
use bevy::utils::HashMap;
use bevy_quinnet::server::{ConnectionEvent, ConnectionLostEvent, Server, ServerConfiguration};
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::shared::channel::ChannelId;
use crate::AppState;
use crate::asset_loader::AssetsLoadedEvent;
use crate::asset_loader::components::SpriteEnum;
use crate::simulation::PlayerData;
use crate::client_networking::ClientMessage;
use crate::simulation::events::{OnObjectDespawnEvent, OnPlayerConnectEvent, OnPlayerSpawnEvent};
use crate::utils::networking::messages::{PhysicsObjData, ServerMessage};
use crate::server_networking::DEFAULT_SERVER_PORT;
use crate::simulation::{Lobby, ObjectId};
use crate::simulation::Object;
use crate::simulation::server_sim::player::{OnHealthChangedEvent, OnKillEvent, Player, PlayerInput, PlayerTurret};
use crate::simulation::SyncedObjects;
use crate::utils::commands::despawn::CustomDespawnExt;
use crate::utils::commands::try_insert::TryInsertExt;

pub fn server_recv(
    mut server: ResMut<Server>,
    mut commands: Commands,
    lobby: Res<Lobby>,
    objects: Res<SyncedObjects>,
) {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients().into_iter() {
        while let Ok(Some(message)) = endpoint.receive_message_from::<ClientMessage>(client_id) {
            match message {
                ClientMessage::InputMessage { input } => {
                    if let Some(data) = lobby.player_data.get(&client_id) &&
                        let Some(object_id) = data.object_id &&
                        let Some(&entity) = objects.objects.get(&object_id) {
                        commands.entity(entity).try_insert(PlayerInput::from(input));
                    }
                }
            }
        }
    }
}

const UNRELIABLE_BYTE_MAX: usize = 3000;

pub fn server_send_phys_obj(
    server: Res<Server>,
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

    objects.iter().par_chunk_map(&TaskPool::new(), chunk_size, |chunk| {
        chunk.iter().cloned().collect::<HashMap<ObjectId, PhysicsObjData>>()
    }).into_iter().for_each(|objects|
        {
            server.endpoint().broadcast_message_on(
                ChannelId::Unreliable,
                ServerMessage::PhysObjUpdate { objects },
            ).unwrap();
        });
}

pub fn server_send_turrets(
    server: Res<Server>,
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
    server.endpoint().broadcast_message_on(
        ChannelId::Unreliable,
        ServerMessage::TurretRotationUpdate { turrets },
    ).unwrap();
}

pub fn update_kill_death_count(
    mut kill_events: EventReader<OnKillEvent>,
    mut lobby: ResMut<Lobby>,
    server: Res<Server>,
) {
    kill_events.iter().for_each(|e| {
        if let Some(mut attacker_data) = lobby.player_data.get_mut(&e.attacker_id) {
            attacker_data.kills += 1;
            server.endpoint().broadcast_message_on(
                ChannelId::UnorderedReliable,
                ServerMessage::PlayerDataUpdate { player_id: e.attacker_id, data: attacker_data.clone() },
            ).unwrap();
        }
        if let Some(victim_data) = lobby.player_data.get_mut(&e.victim_id) {
            victim_data.deaths += 1;
            server.endpoint().broadcast_message_on(
                ChannelId::UnorderedReliable,
                ServerMessage::PlayerDataUpdate { player_id: e.victim_id, data: victim_data.clone() },
            ).unwrap();
        }
    });
}

pub fn update_health(
    server: Res<Server>,
    mut health_events: EventReader<OnHealthChangedEvent>
) {
    health_events.iter().for_each(|e| {
       server.endpoint().broadcast_message_on(
           ChannelId::Unreliable,
           ServerMessage::HealthUpdate {
               object_id: e.object_id,
               health: e.health,
               max_health: e.max_health,
           }
       ).unwrap();
    });
}

pub fn in_game_on_load(
    mut evt: EventReader<AssetsLoadedEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if evt.iter().next().is_some() {
        next_state.set(AppState::InGame);
    }
}

pub fn server_start_listening(mut server: ResMut<Server>) {
    const SERVER_HOSTNAME: &str = "TankRPGHost";

    server.start_endpoint(
        ServerConfiguration::from_ip(Ipv4Addr::new(0, 0, 0, 0).into(), DEFAULT_SERVER_PORT),
        CertificateRetrievalMode::GenerateSelfSigned { server_hostname: SERVER_HOSTNAME.to_string() },
    ).unwrap();
}

pub fn on_object_despawn(
    mut despawn_event: EventReader<OnObjectDespawnEvent>,
    server: Res<Server>,
) {
    despawn_event.iter().for_each(|e| {
        server.endpoint().broadcast_message_on(
            ChannelId::UnorderedReliable,
            ServerMessage::ObjectDespawn { object_id: e.id },
        ).unwrap();
    });
}

pub fn on_client_connect(
    mut connection_events: EventReader<ConnectionEvent>,
    mut spawn_event_writer: EventWriter<OnPlayerConnectEvent>,
    server: Res<Server>,
    mut lobby: ResMut<Lobby>,
    player_query: Query<(&GlobalTransform, &Object), With<Player>>,
) {
    for &ConnectionEvent { id } in connection_events.iter() {
        info!("Player {id} Connected.");

        server.endpoint().broadcast_message_on(
            ChannelId::UnorderedReliable,
            ServerMessage::PlayerConnected {
                player_id: id,
                data: PlayerData::default(),
            }).unwrap();

        server.endpoint().send_message_on(
            id,
            ChannelId::UnorderedReliable,
            ServerMessage::YouConnected { player_id: id },
        ).unwrap();

        for (&player_id, data) in lobby.player_data.iter() {
            server.endpoint().send_message_on(
                id,
                ChannelId::UnorderedReliable,
                ServerMessage::PlayerConnected { player_id, data: data.clone() },
            ).unwrap();

            if let Some(object_id) = data.object_id {
                //TODO inefficient linear search, add objects map to server_networking as well?
                let position = player_query.iter()
                    .find(|(_, obj)| obj.id == object_id)
                    .unwrap().0.translation().truncate();

                server.endpoint().send_message_on(
                    id,
                    ChannelId::UnorderedReliable,
                    ServerMessage::PlayerSpawn { player_id, object_id, position },
                ).unwrap();
            }
        }

        lobby.player_data.insert(id, PlayerData::default());

        spawn_event_writer.send(OnPlayerConnectEvent {
            player_id: id,
        });
    }
}

pub fn on_client_disconnect(
    mut lost_connect_events: EventReader<ConnectionLostEvent>,
    mut commands: Commands,
    server: Res<Server>,
    mut lobby: ResMut<Lobby>,
    objects: ResMut<SyncedObjects>,
) {
    for &ConnectionLostEvent { id } in lost_connect_events.iter() {
        info!("Player {id} Disconnected");
        if let Some(data) = lobby.player_data.remove(&id) &&
            let Some(object_id) = data.object_id &&
            let Some(&entity) = objects.objects.get(&object_id)
        {
            commands.entity(entity).custom_despawn();
        }

        server.endpoint().broadcast_message_on(
            ChannelId::UnorderedReliable,
            ServerMessage::PlayerDisconnected { player_id: id },
        ).unwrap();
    }
}

pub fn on_player_spawn(
    mut spawn_events: EventReader<OnPlayerSpawnEvent>,
    server: Res<Server>,
) {
    spawn_events.iter().for_each(|e| {
        server.endpoint().broadcast_message_on(
            ChannelId::UnorderedReliable,
            ServerMessage::PlayerSpawn {
                player_id: e.player_id,
                object_id: e.object_id,
                position: e.position,
            },
        ).unwrap();
    });
}
