use bevy::prelude::{Children, Commands, Entity, EventReader, NextState, Query, Res, ResMut, Transform, With};
use bevy::log::info;
use std::mem::size_of;
use std::net::{Ipv4Addr};
use bevy::tasks::{ParallelSlice, TaskPool};
use bevy_rapier2d::dynamics::Velocity;
use bevy::utils::HashMap;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Align, Align2};
use bevy_quinnet::server::{ConnectionEvent, ConnectionLostEvent, Server, ServerConfiguration};
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::shared::channel::ChannelId;
use local_ip_address::local_ip;
use crate::asset_loader::AssetsLoadedEvent;
use crate::asset_loader::components::SpriteEnum;
use crate::networking::Lobby;
use crate::networking::client::ClientMessage;
use crate::networking::messages::{PhysicsObjData, ServerMessage};
use crate::networking::server::SERVER_PORT;
use crate::object::ObjectId;
use crate::object::components::Object;
use crate::player::{Player, PlayerTurret, spawn_new_player};
use crate::scenes::AppState;
use crate::utils::CustomDespawn;

pub fn server_recv(
    mut server: ResMut<Server>,
    mut commands: Commands,
    lobby: Res<Lobby>,
) {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients().into_iter() {
        while let Some(message) = endpoint.receive_message_from::<ClientMessage>(client_id).unwrap() {
            match message {
                ClientMessage::InputMessage { input } => {
                    if let Some(player_entity) = lobby.players.get(&client_id) {
                        commands.entity(*player_entity).insert(input);
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
        ServerConfiguration::from_ip(Ipv4Addr::new(0, 0, 0, 0).into(), SERVER_PORT),
        CertificateRetrievalMode::GenerateSelfSigned { server_hostname: SERVER_HOSTNAME.to_string() },
    ).unwrap();
}

pub fn server_ip_display(
    mut egui_ctx: EguiContexts,
) {
    egui::Window::new("Server IP")
        .anchor(Align2([Align::Min, Align::Min]), [0., 0.])
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label(format!("{}:{}", local_ip().unwrap(), SERVER_PORT));
        });
}

pub fn on_client_connect(
    mut connection_events: EventReader<ConnectionEvent>,
    mut commands: Commands,
    server: Res<Server>,
    mut lobby: ResMut<Lobby>,
    mut objects: Query<&mut Object>,
) {
    for &ConnectionEvent { id } in connection_events.iter() {
        info!("Player {id} Connected.");
        let new_player_entity: Entity = spawn_new_player(&mut commands, id, None);
        let new_object = Object::new();
        commands.entity(new_player_entity).insert(new_object);
        for (&player_id, &player) in lobby.players.iter() {
            let object_id = objects.get_mut(player).unwrap().id;
            server.endpoint().send_message_on(
                id,
                ChannelId::UnorderedReliable,
                ServerMessage::PlayerConnected { player_id, object_id },
            ).unwrap();
        }

        lobby.players.insert(id, new_player_entity);

        server.endpoint().broadcast_message_on(
            ChannelId::UnorderedReliable,
            ServerMessage::PlayerConnected {
                player_id: id,
                object_id: new_object.id,
            }).unwrap();
    }
}

pub fn on_client_disconnect(
    mut lost_connect_events: EventReader<ConnectionLostEvent>,
    mut commands: Commands,
    server: Res<Server>,
    mut lobby: ResMut<Lobby>,
) {
    for &ConnectionLostEvent { id } in lost_connect_events.iter() {
        info!("Player {id} Disconnected");
        if let Some(player_entity) = lobby.players.remove(&id) {
            commands.entity(player_entity).custom_despawn();
        }

        server.endpoint().broadcast_message_on(
            ChannelId::UnorderedReliable,
            ServerMessage::PlayerDisconnected { player_id: id },
        ).unwrap();
    }
}
