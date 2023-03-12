pub mod ui;

use bevy::prelude::{Children, Commands, EventReader, EventWriter, NextState, Query, Res, ResMut, Transform, With};
use bevy::log::info;
use std::mem::size_of;
use std::net::Ipv4Addr;
use bevy::tasks::{ParallelSlice, TaskPool};
use bevy_rapier2d::dynamics::Velocity;
use bevy::utils::HashMap;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Align2;
use bevy_quinnet::server::{ConnectionEvent, ConnectionLostEvent, Server, ServerConfiguration};
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::shared::channel::ChannelId;
use local_ip_address::local_ip;
use crate::asset_loader::AssetsLoadedEvent;
use crate::asset_loader::components::SpriteEnum;
use crate::networking::{Lobby, PlayerData};
use crate::networking::client::{ClientMessage};
use crate::networking::messages::{PhysicsObjData, ServerMessage};
use crate::networking::server::SERVER_PORT;
use crate::networking::server::events::{OnObjectDespawnEvent, OnPlayerConnectEvent, OnPlayerSpawnEvent};
use crate::networking::server::systems::ui::ServerVisualizer;
use crate::object::ObjectId;
use crate::object::components::Object;
use crate::player::{Player, PlayerTurret};
use crate::scenes::AppState;
use crate::utils::despawn::CustomDespawnExt;
use crate::utils::TryInsertExt;

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
                    if let Some(player_entity) = lobby.get_entity(&client_id) {
                        commands.entity(player_entity).try_insert(input);
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
) {
    for &ConnectionLostEvent { id } in lost_connect_events.iter() {
        info!("Player {id} Disconnected");
        if let Some(data) = lobby.player_data.remove(&id) {
            if let Some(entity) = data.entity {
                commands.entity(entity).custom_despawn();
            }
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
            },
        ).unwrap();
    });
}

pub fn server_stats_egui(
    mut egui_ctx: EguiContexts,
    mut client_join: EventReader<ConnectionEvent>,
    mut client_leave: EventReader<ConnectionLostEvent>,
    visualizer: Option<ResMut<ServerVisualizer<512>>>,
    mut _commands: Commands,
    lobby: Res<Lobby>,
    server: Res<Server>,
) {
    let Some(mut visualizer) = visualizer else {
        _commands.insert_resource(ServerVisualizer::<512>::default());
        return;
    };

    client_join.iter().for_each(|ConnectionEvent { id }| {
        visualizer.add_client(*id);
    });
    client_leave.iter().for_each(|ConnectionLostEvent { id }| {
        visualizer.remove_client(*id);
    });

    visualizer.update(&server);

    egui::Window::new("Server Stats")
        .anchor(Align2::LEFT_TOP, [0., 0.])
        .collapsible(true)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Server IP:");
                let server_ip = format!("{}:{}", local_ip().unwrap(), SERVER_PORT);
                ui.monospace(server_ip.clone());
                if ui.small_button("📋").clicked() {
                    ui.output_mut(|o| o.copied_text = server_ip);
                }
            });

            ui.separator();

            ui.label("Player Lobby");
            ui.group(|ui| {
                lobby.player_data.iter().for_each(|player| {
                    ui.label(format!("Player {}: Entity {:?}", player.0, player.1.clone()));
                });
            });

            ui.separator();
            visualizer.show_window(ui);
        });
}
