use bevy::prelude::{EventReader, EventWriter, Res, ResMut};
use bevy_quinnet::client::Client;
use bevy_quinnet::shared::channel::ChannelId;
use crate::client_networking::{ClientMessage, RecvHealthUpdateEvent, RecvMatchTimeEvent, RecvObjectDespawnEvent, RecvPhysObjUpdateEvent, RecvPlayerConnectEvent, RecvPlayerDataUpdateEvent, RecvPlayerLeaveEvent, RecvPlayerSpawnEvent, RecvYouConnectEvent};
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

pub fn client_recv_all(
    mut client: ResMut<Client>,
    mut unreliable_writer: EventWriter<ServerUnreliableMessage>,
    mut reliable_writer: EventWriter<ServerReliableMessage>,
    mut init_writer: EventWriter<ServerInitMessage>,
) {
    while let Ok(Some(message)) = client.connection_mut().receive_message::<ServerMessage>() {
        match message {
            ServerMessage::Unreliable(msg) => {
                unreliable_writer.send(msg);
            }
            ServerMessage::Reliable(msg) => {
                reliable_writer.send(msg);
            }
            ServerMessage::Init(msg) => {
                init_writer.send(msg);
            }
        }
    }
}

pub fn client_recv_unreliable(
    mut messages: EventReader<ServerUnreliableMessage>,
    mut player_data_event: EventWriter<RecvPlayerDataUpdateEvent>,
    mut health_event: EventWriter<RecvHealthUpdateEvent>,
    mut phys_update_event: EventWriter<RecvPhysObjUpdateEvent>,
    mut match_time_event: EventWriter<RecvMatchTimeEvent>,
) {
    messages.iter().for_each(|e| {
        let e = e.clone();
        e.player_data.into_iter().for_each(|(id, data)| {
            player_data_event.send(RecvPlayerDataUpdateEvent { id, data });
        });
        e.healths.into_iter().for_each(|(object_id, health)| {
            health_event.send(RecvHealthUpdateEvent {
                object_id,
                health: health.health,
                max_health: health.max_health,
            });
        });

        e.object_data.into_iter().for_each(|(id, data)| {
            phys_update_event.send(RecvPhysObjUpdateEvent { id, data });
        });
        // e.turret_rotation_data.into_iter().for_each(|(parent_id, rotation)| {
        //     turret_update_event.send(RecvTurretUpdateEvent { turret_id: parent_id, rotation });
        // });
        if let Some(time_remaining) = e.match_timer {
            match_time_event.send(RecvMatchTimeEvent { time_remaining });
        }
    });
}

pub fn client_recv_reliable(
    mut messages: EventReader<ServerReliableMessage>,
    mut join_event: EventWriter<RecvPlayerConnectEvent>,
    mut leave_event: EventWriter<RecvPlayerLeaveEvent>,
    mut spawn_event: EventWriter<RecvPlayerSpawnEvent>,
    mut despawn_event: EventWriter<RecvObjectDespawnEvent>,
) {
    messages.iter().for_each(|e| {
        let e = e.clone();
        e.connect_events.into_iter().for_each(|(player_id, data)| {
            join_event.send(RecvPlayerConnectEvent { player_id, data });
        });
        e.disconnect_events.into_iter().for_each(|player_id| {
            leave_event.send(RecvPlayerLeaveEvent { player_id });
        });
        e.player_spawn_events.into_iter().for_each(|e| {
            spawn_event.send(RecvPlayerSpawnEvent {
                player_id: e.player_id,
                turret_object_ids: e.turret_object_ids,
                object_id: e.object_id,
                position: e.position,
            });
        });
        e.despawn_events.into_iter().for_each(|object_id| {
            despawn_event.send(RecvObjectDespawnEvent { object_id });
        });
    });
}

pub fn client_recv_init(
    mut messages: EventReader<ServerInitMessage>,
    mut you_joined_event: EventWriter<RecvYouConnectEvent>,
    mut spawn_event: EventWriter<RecvPlayerSpawnEvent>,
) {
    messages.iter().for_each(|e| {
        you_joined_event.send(RecvYouConnectEvent { player_id: e.you_connect_event.0 });
        e.existing_players.iter().for_each(|e| {
            spawn_event.send(RecvPlayerSpawnEvent {
                player_id: e.player_id,
                turret_object_ids: e.turret_object_ids.clone(),
                object_id: e.object_id,
                position: e.position,
            });
        });
    });
}
