use bevy::prelude::{Commands, EventReader, EventWriter, Query, Res, ResMut, State, With};
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bevy::log::info;
use bevy::time::Time;
use bevy_rapier2d::prelude::Velocity;
use crate::asset_loader::AssetsLoadedEvent;
use crate::player::components::PlayerInput;
use crate::networking::{Lobby, ObjectDespawnEvent, PhysObjUpdateEvent, PlayerConnectEvent, PlayerLeaveEvent, TurretUpdateEvent};
use crate::networking::client::ClientInputMessage;
use crate::networking::messages::{ReliableMessages, UnreliableMessages};
use crate::object::SyncedObjects;
use crate::player::{calc_player_next_velocity, Player, You};
use crate::scenes::AppState;
use crate::utils::CustomDespawn;

pub fn client_send(
    input: Res<PlayerInput>,
    mut client: ResMut<RenetClient>,
) {
    let message = ClientInputMessage {
        input: input.clone(),
    };

    let bin_message = bincode::serialize(&message).unwrap();
    client.send_message(DefaultChannel::Unreliable, bin_message);
}

pub fn client_recv(
    mut client: ResMut<RenetClient>,
    mut join_event: EventWriter<PlayerConnectEvent>,
    mut leave_event: EventWriter<PlayerLeaveEvent>,
    mut despawn_event: EventWriter<ObjectDespawnEvent>,
    mut phys_update_event: EventWriter<PhysObjUpdateEvent>,
    mut turr_update_event: EventWriter<TurretUpdateEvent>
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        let server_message: ReliableMessages = bincode::deserialize(&message).unwrap();
        match server_message {
            ReliableMessages::PlayerConnected { player_id, object_id } => {
                join_event.send(PlayerConnectEvent { player_id, object_id });
            }
            ReliableMessages::PlayerDisconnected { player_id } => {
                leave_event.send(PlayerLeaveEvent { player_id });
            }
            ReliableMessages::ObjectDespawn { object_id } => {
                despawn_event.send(ObjectDespawnEvent { object_id });
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
    mut lobby: ResMut<Lobby>,
) {
    for ev in leave_events.iter() {
        info!("Player {} Disconnected", ev.player_id);
        if let Some(player_entity) = lobby.players.remove(&ev.player_id) {
            commands.entity(player_entity).custom_despawn();
        }
    }
}

pub fn on_object_despawn (
    mut events: EventReader<ObjectDespawnEvent>,
    objects: Res<SyncedObjects>,
    mut commands: Commands,
) {
    events.iter().for_each(|event| {
        info!("Recieved despawn from server");
        if let Some(&ent) = objects.objects.get(&event.object_id) {
            commands.entity(ent).custom_despawn();
        }
    });
}

pub fn prediction_move(
    mut query: Query<(&mut Velocity, &Player, &PlayerInput), With<You>>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(|(mut vel, player, input)| {
        vel.linvel = calc_player_next_velocity(vel.linvel, player, input, time.delta_seconds());
    });
}

pub fn main_menu_on_load(
    mut evt: EventReader<AssetsLoadedEvent>,
    mut state: ResMut<State<AppState>>,
) {
    evt.iter().for_each(|_| {
        state.set(AppState::MainMenu).unwrap();
    })
}