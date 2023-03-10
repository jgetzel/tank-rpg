use bevy::prelude::{Commands, EventReader, EventWriter, NextState, Query, Res, ResMut, With};
use bevy::log::info;
use bevy::time::Time;
use bevy_quinnet::client::Client;
use bevy_quinnet::shared::channel::ChannelId;
use bevy_rapier2d::prelude::Velocity;
use crate::asset_loader::AssetsLoadedEvent;
use crate::player::components::PlayerInput;
use crate::networking::{Lobby, ObjectDespawnEvent, PhysObjUpdateEvent, PlayerConnectEvent, PlayerLeaveEvent, TurretUpdateEvent};
use crate::networking::client::{ClientId, ClientMessage, YouConnectEvent};
use crate::networking::messages::*;
use crate::object::SyncedObjects;
use crate::player::{calc_player_next_velocity, Player, You};
use crate::scenes::AppState;
use crate::utils::CustomDespawn;

pub fn client_send(
    input: Res<PlayerInput>,
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
    mut you_joined_event: EventWriter<YouConnectEvent>,
    mut join_event: EventWriter<PlayerConnectEvent>,
    mut leave_event: EventWriter<PlayerLeaveEvent>,
    mut despawn_event: EventWriter<ObjectDespawnEvent>,
    mut phys_update_event: EventWriter<PhysObjUpdateEvent>,
    mut turr_update_event: EventWriter<TurretUpdateEvent>,
) {
    while let Ok(Some(message)) = client.connection_mut().receive_message::<ServerMessage>() {
        match message {
            ServerMessage::YouConnected { player_id } => {
                you_joined_event.send(YouConnectEvent { player_id });
            }
            ServerMessage::PlayerConnected { player_id, object_id } => {
                join_event.send(PlayerConnectEvent { player_id, object_id });
            }
            ServerMessage::PlayerDisconnected { player_id } => {
                leave_event.send(PlayerLeaveEvent { player_id });
            }
            ServerMessage::ObjectDespawn { object_id } => {
                despawn_event.send(ObjectDespawnEvent { object_id });
            }
            ServerMessage::PhysObjUpdate { objects } => {
                objects.into_iter().for_each(|(id, data)| {
                    phys_update_event.send(PhysObjUpdateEvent { id, data })
                })
            }
            ServerMessage::TurretRotationUpdate { turrets } => {
                turrets.into_iter().for_each(|(parent_id, rotation)| {
                    turr_update_event.send(TurretUpdateEvent { parent_id, rotation });
                })
            }
        }
    }
}

pub fn on_you_joined(
    mut you_join_events: EventReader<YouConnectEvent>,
    mut commands: Commands,
) {
    you_join_events.iter().for_each(|e| {
       commands.insert_resource(ClientId(e.player_id));
    });
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

pub fn on_object_despawn(
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
    mut next_state: ResMut<NextState<AppState>>,
) {
    if evt.iter().next().is_some() {
        next_state.set(AppState::MainMenu);
    }
}