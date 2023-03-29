use bevy::prelude::{Children, Commands, EventReader, EventWriter, GlobalTransform, Query, Res, ResMut, Transform, With};
use bevy_rapier2d::dynamics::Velocity;
use bevy_quinnet::server::{ConnectionEvent, ConnectionLostEvent, Server};
use bevy_quinnet::shared::channel::ChannelId;
use crate::asset_loader::components::SpriteEnum;
use crate::simulation::{PlayerData};
use crate::client_networking::ClientMessage;
use crate::simulation::events::{OnObjectDespawnEvent, OnPlayerConnectEvent, OnPlayerDisconnectEvent, OnPlayerSpawnEvent};
use crate::utils::networking::messages::{PhysicsObjData, ServerInitMessage, ServerMessage, ServerReliableMessage, ServerUnreliableMessage};
use crate::simulation::{Lobby};
use crate::simulation::Object;
use crate::simulation::server_sim::match_ffa::{MatchTimer};
use crate::simulation::server_sim::player::{Health, Player, PlayerInput, PlayerTurret};
use crate::simulation::SyncedObjects;
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

pub fn server_send_unreliable(
    server: ResMut<Server>,
    lobby: Res<Lobby>,
    health_query: Query<(&Object, &Health)>,
    object_query: Query<(&Transform, Option<&Velocity>, Option<&SpriteEnum>, &Object)>,
    match_timer: Option<Res<MatchTimer>>,
) {
    let message = ServerUnreliableMessage {
        player_data: lobby.player_data.clone(),
        healths: health_query.iter()
            .map(|(object, health)|
                (object.id, health.clone())
            ).collect(),
        object_data: object_query.iter().map(
            |(&trans, vel, sprite, obj)| {
                let data = PhysicsObjData {
                    transform: trans,
                    velocity: vel.unwrap_or(&Velocity::zero()).linvel,
                    sprite: sprite.copied(),
                };

                (obj.id, data)
            }).collect(),
        match_timer: match_timer.map(|timer| timer.time_remaining),
    };

    server.endpoint().broadcast_message_on(
        ChannelId::UnorderedReliable,
        ServerMessage::Unreliable(message),
    ).unwrap();
}

pub fn server_send_reliable(
    server: ResMut<Server>,
    mut connect_event: EventReader<ConnectionEvent>,
    mut connect_writer: EventWriter<OnPlayerConnectEvent>,
    mut disconnect_event: EventReader<ConnectionLostEvent>,
    mut disconnect_writer: EventWriter<OnPlayerDisconnectEvent>,
    mut spawn_event: EventReader<OnPlayerSpawnEvent>,
    mut despawn_event: EventReader<OnObjectDespawnEvent>,
) {
    let mut message = ServerReliableMessage::default();

    connect_event.iter().for_each(|e| {
        message.connect_events.push((e.id, PlayerData::default()));
        connect_writer.send(OnPlayerConnectEvent { player_id: e.id });
    });

    disconnect_event.iter().for_each(|e| {
        message.disconnect_events.push(e.id);
        disconnect_writer.send(OnPlayerDisconnectEvent { player_id: e.id });
    });

    spawn_event.iter().for_each(|e| {
        message.player_spawn_events.push(e.clone());
    });

    despawn_event.iter().for_each(|e| {
        message.despawn_events.push(e.id);
    });

    server.endpoint().broadcast_message_on(
        ChannelId::UnorderedReliable,
        ServerMessage::Reliable(message),
    ).unwrap();
}

pub fn server_send_init_player(
    server: ResMut<Server>,
    mut connect_event_reader: EventReader<OnPlayerConnectEvent>,
    player_query: Query<(&Player, &GlobalTransform, &Object, &Children)>,
    turr_query: Query<&Object, With<PlayerTurret>>,
) {
    connect_event_reader.iter().for_each(|e| {
        let message = ServerInitMessage {
            you_connect_event: (e.player_id, PlayerData::default()),
            existing_players: player_query.iter()
                .map(|(player, trans, obj, children)| {
                    OnPlayerSpawnEvent {
                        player_id: player.id,
                        turret_object_ids: children.iter()
                            .filter_map(|e|
                                match turr_query.get(*e) {
                                    Ok(turr) => Some(turr.id),
                                    Err(_) => None,
                                }
                            ).collect(),
                        object_id: obj.id,
                        position: trans.translation().truncate(),
                    }
                }).collect(),
        };

        server.endpoint().send_message_on(
            e.player_id,
            ChannelId::UnorderedReliable,
            ServerMessage::Init(message),
        ).unwrap();
    });
}