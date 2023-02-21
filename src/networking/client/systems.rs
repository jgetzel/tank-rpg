use bevy::prelude::{Commands, EventReader, EventWriter, Res, ResMut};
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bevy::log::info;
use bevy::hierarchy::DespawnRecursiveExt;
use crate::client_input::PlayerInput;
use crate::networking::{Lobby, PhysObjUpdateEvent, PlayerJoinEvent, PlayerLeaveEvent};
use crate::networking::client::ClientInputMessage;
use crate::networking::client::resources::RequestIdCounter;
use crate::networking::messages::{ReliableMessages, UnreliableMessages};

pub fn client_send(
    input: Res<PlayerInput>,
    mut client: ResMut<RenetClient>,
    mut request_id: ResMut<RequestIdCounter>,
) {
    let message = ClientInputMessage {
        input: input.clone(),
        request_id: request_id.next_id(),
    };

    let bin_message = bincode::serialize(&message).unwrap();
    client.send_message(DefaultChannel::Unreliable, bin_message);
}

pub fn client_recv(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    mut join_event: EventWriter<PlayerJoinEvent>,
    mut leave_event: EventWriter<PlayerLeaveEvent>,
    mut update_event: EventWriter<PhysObjUpdateEvent>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        let server_message: ReliableMessages = bincode::deserialize(&message).unwrap();
        match server_message {
            ReliableMessages::PlayerConnected { player_id, object_id } => {
                join_event.send(PlayerJoinEvent { player_id, object_id });
            }
            ReliableMessages::PlayerDisconnected { player_id } => {
                leave_event.send(PlayerLeaveEvent { player_id })
            }
        };
    }

    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let server_message: UnreliableMessages = bincode::deserialize(&message).unwrap();
        match server_message {
            UnreliableMessages::PhysObjUpdate { objects } => {
                for (id, data) in objects.into_iter() {
                    update_event.send(PhysObjUpdateEvent { id, data });
                }
            }
        }
    }
}

pub fn on_player_leave(
    mut leave_events: EventReader<PlayerLeaveEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>
) {
    for ev in leave_events.iter() {
        info!("Player {} Disconnected", ev.player_id);
        if let Some(player_entity) = lobby.players.remove(&ev.player_id) {
            commands.entity(player_entity).despawn_recursive();
        }
    }
}
