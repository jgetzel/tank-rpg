use bevy::prelude::{Commands, EventReader, EventWriter, NextState, Res, ResMut};
use bevy::log::info;
use bevy_quinnet::client::Client;
use bevy_quinnet::shared::channel::ChannelId;
use crate::asset_loader::AssetsLoadedEvent;
use crate::player::components::PlayerInput;
use crate::networking::{Lobby, ObjectDespawnEvent, PhysObjUpdateEvent, PlayerConnectEvent, PlayerData, PlayerLeaveEvent, TurretUpdateEvent};
use crate::networking::client::{ClientId, ClientMessage, YouConnectEvent};
use crate::networking::messages::*;
use crate::object::SyncedObjects;
use crate::scenes::AppState;
use crate::utils::despawn::CustomDespawnExt;

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
        if let Some(PlayerData { entity: Some(entity) }) = lobby.player_data.remove(&ev.player_id) {
            commands.entity(entity).custom_despawn();
        }
    }
}

pub fn on_object_despawn(
    mut events: EventReader<ObjectDespawnEvent>,
    objects: Res<SyncedObjects>,
    mut commands: Commands,
) {
    events.iter().for_each(|event| {
        info!("Received despawn from server");
        if let Some(&ent) = objects.objects.get(&event.object_id) {
            commands.entity(ent).custom_despawn();
        }
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