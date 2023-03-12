use bevy::prelude::{BuildChildren, Commands, EventReader, EventWriter, NextState, Res, ResMut};
use bevy::log::info;
use bevy_egui::{egui, EguiContexts};
use bevy_quinnet::client::Client;
use bevy_quinnet::shared::channel::ChannelId;
use crate::asset_loader::AssetsLoadedEvent;
use crate::player::components::PlayerInput;
use crate::networking::{Lobby, PlayerData};
use crate::networking::client::{ClientId, ClientMessage, RecvObjectDespawnEvent, RecvPhysObjUpdateEvent, RecvPlayerConnectEvent, RecvPlayerLeaveEvent, RecvTurretUpdateEvent, RecvYouConnectEvent};
use crate::networking::messages::*;
use crate::object::{Object, SyncedObjects};
use crate::networking::client::RecvPlayerSpawnEvent;
use crate::player::bundles::{get_player_bundle, get_turret_bundle};
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
    (mut you_joined_event, mut join_event, mut leave_event):
    (
        EventWriter<RecvYouConnectEvent>, EventWriter<RecvPlayerConnectEvent>, EventWriter<RecvPlayerLeaveEvent>,
    ),
    mut despawn_event: EventWriter<RecvObjectDespawnEvent>,
    mut spawn_event: EventWriter<RecvPlayerSpawnEvent>,
    mut phys_update_event: EventWriter<RecvPhysObjUpdateEvent>,
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
            ServerMessage::PlayerSpawn { player_id, object_id } => {
                spawn_event.send(RecvPlayerSpawnEvent { player_id, object_id });
            }
            ServerMessage::ObjectDespawn { object_id } => {
                despawn_event.send(RecvObjectDespawnEvent { object_id });
            }
            ServerMessage::PhysObjUpdate { objects } => {
                objects.into_iter().for_each(|(id, data)| {
                    phys_update_event.send(RecvPhysObjUpdateEvent { id, data })
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
    lobby: ResMut<Lobby>,
    objects: ResMut<SyncedObjects>,
) {
    for ev in leave_events.iter() {
        info!("Player {} Disconnected", ev.player_id);
        if let Some(data) = lobby.player_data.get(&ev.player_id) &&
            let Some(object_id) = data.object_id &&
            let Some(&entity) = objects.objects.get(&object_id) {
            commands.entity(entity).custom_despawn();
        }
    }
}

pub fn on_player_spawn(
    mut spawn_event: EventReader<RecvPlayerSpawnEvent>,
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

        commands.entity(entity).insert(get_player_bundle(e.player_id, None))
            .insert(Object { id: e.object_id })
            .with_children(|p| {
                p.spawn(get_turret_bundle());
            });

        if let Some(mut data) = lobby.player_data.get_mut(&e.player_id) {
            data.object_id = Some(e.object_id);
        } else {
            let mut data = PlayerData::default();
            data.object_id = Some(e.object_id);
            lobby.player_data.insert(e.player_id, data);
        }
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

pub fn main_menu_on_load(
    mut evt: EventReader<AssetsLoadedEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if evt.iter().next().is_some() {
        next_state.set(AppState::MainMenu);
    }
}

pub fn show_player_lobby(
    mut egui_ctx: EguiContexts,
    lobby: Res<Lobby>,
) {
    egui::Window::new("Lobby")
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.group(|ui| {
                lobby.player_data.iter().for_each(|player| {
                    ui.label(format!("Player {}: Entity {:?}", player.0, player.1.clone()));
                });
            });
        });
}