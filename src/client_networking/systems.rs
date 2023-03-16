use bevy::prelude::{EventWriter, Res, ResMut};
use bevy_quinnet::client::Client;
use bevy_quinnet::shared::channel::ChannelId;
use crate::client_networking::{ClientMessage, RecvHealthUpdateEvent, RecvObjectDespawnEvent, RecvPhysObjUpdateEvent, RecvPlayerConnectEvent, RecvPlayerDataUpdateEvent, RecvPlayerLeaveEvent, RecvPlayerSpawnEvent, RecvTurretUpdateEvent, RecvYouConnectEvent};
use crate::client_networking::client_input::ClientInput;
use crate::utils::networking::messages::*;

pub fn client_send(
    input: Res<ClientInput>,
    client: Res<Client>,
) {
    client.connection().send_message_on(
        ChannelId::Unreliable,
        ClientMessage::InputMessage {
            input: input.clone(),
        }).unwrap();
}

pub fn client_recv(
    mut client: ResMut<Client>,
    (mut you_joined_event, mut join_event, mut leave_event):
    (
        EventWriter<RecvYouConnectEvent>,
        EventWriter<RecvPlayerConnectEvent>,
        EventWriter<RecvPlayerLeaveEvent>,
    ),
    (mut despawn_event, mut spawn_event):
    (
        EventWriter<RecvObjectDespawnEvent>,
        EventWriter<RecvPlayerSpawnEvent>
    ),
    (mut phys_update_event, mut health_update_event, mut player_data_event):
    (
        EventWriter<RecvPhysObjUpdateEvent>,
        EventWriter<RecvHealthUpdateEvent>,
        EventWriter<RecvPlayerDataUpdateEvent>
    ),
    mut turr_update_event: EventWriter<RecvTurretUpdateEvent>,
) {
    while let Ok(Some(message)) = client.connection_mut().receive_message::<ServerMessage>() {
        match message {
            ServerMessage::YouConnected { player_id } => {
                you_joined_event.send(RecvYouConnectEvent { player_id });
            }
            ServerMessage::PlayerConnected { player_id, data } => {
                join_event.send(RecvPlayerConnectEvent { player_id, data });
            }
            ServerMessage::PlayerDisconnected { player_id } => {
                leave_event.send(RecvPlayerLeaveEvent { player_id });
            }
            ServerMessage::PlayerSpawn { player_id, object_id, position } => {
                spawn_event.send(RecvPlayerSpawnEvent { player_id, object_id, position });
            }
            ServerMessage::ObjectDespawn { object_id } => {
                despawn_event.send(RecvObjectDespawnEvent { object_id });
            }
            ServerMessage::PhysObjUpdate { objects } => {
                objects.into_iter().for_each(|(id, data)| {
                    phys_update_event.send(RecvPhysObjUpdateEvent { id, data })
                });
            }
            ServerMessage::HealthUpdate { object_id, health, max_health } => {
                health_update_event.send(RecvHealthUpdateEvent {
                    object_id,
                    health,
                    max_health,
                });
            }
            ServerMessage::PlayerDataUpdate { player_id, data } => {
                player_data_event.send(RecvPlayerDataUpdateEvent {
                    id: player_id,
                    data,
                });
            }
            ServerMessage::TurretRotationUpdate { turrets } => {
                turrets.into_iter().for_each(|(parent_id, rotation)| {
                    turr_update_event.send(RecvTurretUpdateEvent { parent_id, rotation });
                })
            }
        }
    }
}