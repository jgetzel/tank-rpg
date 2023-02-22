use bevy::prelude::{Commands, EventReader, EventWriter, Query, Res, ResMut, With};
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bevy::log::info;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::time::Time;
use bevy_rapier2d::prelude::Velocity;
use crate::client_input::PlayerInput;
use crate::networking::{Lobby, PhysObjUpdateEvent, PlayerJoinEvent, PlayerLeaveEvent, TurretUpdateEvent};
use crate::networking::client::ClientInputMessage;
use crate::networking::client::resources::RequestIdCounter;
use crate::networking::messages::{ReliableMessages, UnreliableMessages};
use crate::player::{calc_player_next_velocity, Player, You};

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
    mut client: ResMut<RenetClient>,
    mut join_event: EventWriter<PlayerJoinEvent>,
    mut leave_event: EventWriter<PlayerLeaveEvent>,
    mut phys_update_event: EventWriter<PhysObjUpdateEvent>,
    mut turr_update_event: EventWriter<TurretUpdateEvent>
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        let server_message: ReliableMessages = bincode::deserialize(&message).unwrap();
        match server_message {
            ReliableMessages::PlayerConnected { player_id, object_id } => {
                join_event.send(PlayerJoinEvent { player_id, object_id });
            }
            ReliableMessages::PlayerDisconnected { player_id } => {
                leave_event.send(PlayerLeaveEvent { player_id });
            }
        };
    }

    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let server_message: UnreliableMessages = bincode::deserialize(&message).unwrap();
        match server_message {
            UnreliableMessages::PhysObjUpdate { objects } => {
                for (id, data) in objects.into_iter() {
                    phys_update_event.send(PhysObjUpdateEvent { id, data });
                }
            }
            UnreliableMessages::TurretRotationUpdate { turrets } => {
                turrets.into_iter().for_each(|(parent_id,rotation)| {
                   turr_update_event.send( TurretUpdateEvent { parent_id, rotation });
                });
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

pub fn prediction_move(
    mut query: Query<(&mut Velocity, &Player, &PlayerInput), With<You>>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(|(mut vel, player, input)| {
        vel.linvel = calc_player_next_velocity(vel.linvel, player, input, time.delta_seconds());
    });
}
