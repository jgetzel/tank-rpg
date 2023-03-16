use bevy::prelude::{BuildChildren, Commands, EventReader, EventWriter, Query, Res, ResMut};
use bevy::log::info;
use bevy_quinnet::client::Client;
use bevy_quinnet::shared::channel::ChannelId;
use crate::client_networking::{ClientId, ClientMessage, RecvHealthUpdateEvent, RecvObjectDespawnEvent, RecvPhysObjUpdateEvent, RecvPlayerConnectEvent, RecvPlayerLeaveEvent, RecvPlayerSpawnEvent, RecvTurretUpdateEvent, RecvYouConnectEvent};
use crate::client_networking::client_input::ClientInput;
use crate::utils::networking::messages::*;
use crate::simulation::events::OnPlayerSpawnEvent;
use crate::simulation::server_sim::player::Health;
use crate::simulation::{Lobby, Object, SyncedObjects};
use crate::utils::commands::despawn::CustomDespawnExt;
use crate::simulation::PlayerData;
use crate::utils::prefabs::{get_player_bundle, get_turret_bundle};

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
    (mut phys_update_event, mut health_update_event):
    (
        EventWriter<RecvPhysObjUpdateEvent>,
        EventWriter<RecvHealthUpdateEvent>
    ),
    mut turr_update_event: EventWriter<RecvTurretUpdateEvent>,
    mut lobby: ResMut<Lobby>,
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
                *lobby.player_data.get_mut(&player_id).unwrap() = data.clone();
            }
            ServerMessage::TurretRotationUpdate { turrets } => {
                turrets.into_iter().for_each(|(parent_id, rotation)| {
                    turr_update_event.send(RecvTurretUpdateEvent { parent_id, rotation });
                })
            }
        }
    }
}

pub fn on_you_joined(
    mut you_join_events: EventReader<RecvYouConnectEvent>,
    mut commands: Commands,
) {
    you_join_events.iter().for_each(|e| {
        commands.insert_resource(ClientId(e.player_id));
    });
}

pub fn on_player_join(
    mut join_ev: EventReader<RecvPlayerConnectEvent>,
    mut lobby: ResMut<Lobby>,
) {
    join_ev.iter().for_each(|ev| {
        info!("Player {} Connected", ev.player_id);
        lobby.player_data.insert(ev.player_id, ev.data.clone());
    });
}

pub fn on_player_leave(
    mut leave_events: EventReader<RecvPlayerLeaveEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    objects: ResMut<SyncedObjects>,
) {
    for ev in leave_events.iter() {
        info!("Player {} Disconnected", ev.player_id);
        if let Some(data) = lobby.player_data.remove(&ev.player_id) &&
            let Some(object_id) = data.object_id &&
            let Some(&entity) = objects.objects.get(&object_id) {
            commands.entity(entity).custom_despawn();
        }
    }
}

pub fn on_player_spawn(
    mut spawn_event: EventReader<RecvPlayerSpawnEvent>,
    mut spawn_writer: EventWriter<OnPlayerSpawnEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut objects: ResMut<SyncedObjects>,
) {
    spawn_event.iter().for_each(|e| {
        let entity = match objects.objects.get(&e.object_id) {
            Some(&entity) => entity,
            None => {
                let ent = commands.spawn_empty().id();
                objects.objects.insert(e.object_id, ent);
                ent
            }
        };

        commands.entity(entity).insert(get_player_bundle(e.player_id, Some(e.position)))
            .insert(Object { id: e.object_id })
            .with_children(|p| {
                p.spawn(get_turret_bundle());
            });

        if let Some(mut data) = lobby.player_data.get_mut(&e.player_id) {
            data.object_id = Some(e.object_id);
        } else {
            lobby.player_data.insert(e.player_id, PlayerData::new(e.object_id));
        }

        spawn_writer.send(OnPlayerSpawnEvent {
            player_id: e.player_id,
            object_id: e.object_id,
            position: e.position,
        });
    });
}

pub fn on_health_update(
    mut events: EventReader<RecvHealthUpdateEvent>,
    mut health_q: Query<&mut Health>,
    objects: Res<SyncedObjects>,
) {
    events.iter().for_each(|e| {
        let Some(&entity) = objects.objects.get(&e.object_id) else { return; };
        let Ok(mut health) = health_q.get_mut(entity) else { return; };
        health.max_health = e.max_health;
        health.health = e.health;
    });
}

pub fn on_object_despawn(
    mut events: EventReader<RecvObjectDespawnEvent>,
    objects: Res<SyncedObjects>,
    mut commands: Commands,
) {
    events.iter().for_each(|event| {
        if let Some(&ent) = objects.objects.get(&event.object_id) {
            commands.entity(ent).custom_despawn();
        }
    });
}