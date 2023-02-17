use bevy::prelude::{Commands, EventWriter, Res, ResMut};
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bevy::log::info;
use bevy::hierarchy::DespawnRecursiveExt;
use crate::input_helper::PlayerInput;
use crate::networking::{Lobby, PhysObjUpdateEvent, PlayerJoinEvent, PlayerLeaveEvent};
use crate::networking::messages::{ReliableMessages, UnreliableMessages};

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
    mut update_event: EventWriter<PhysObjUpdateEvent>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        let server_message: ReliableMessages = bincode::deserialize(&message).unwrap();
        match server_message {
            ReliableMessages::PlayerConnected { player_id, object_id } => {
                join_event.send(PlayerJoinEvent { player_id, object_id });
            }
            ReliableMessages::PlayerDisconnected { player_id } => {
                on_player_leave(player_id, &mut commands, &mut lobby);
                leave_event.send(PlayerLeaveEvent { player_id })
            }
        };
    }

    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let server_message: UnreliableMessages = bincode::deserialize(&message).unwrap();
        match server_message {
            UnreliableMessages::PhysObjUpdate { objects } => {
                for (id, data) in objects.into_iter() {
                    update_event.send( PhysObjUpdateEvent { id, data });
                }
            }
        }
    }
}

fn on_player_leave(id: u64, commands: &mut Commands, lobby: &mut Lobby) {
    info!("Player {id} Disconnected");
    if let Some(player_entity) = lobby.players.remove(&id) {
        commands.entity(player_entity).despawn_recursive();
    }
}