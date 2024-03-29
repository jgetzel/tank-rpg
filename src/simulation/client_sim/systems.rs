use crate::asset_loader::components::SpriteEnum;
use crate::simulation::{Lobby, Object, PlayerData};
use crate::simulation::SyncedObjects;
use bevy::prelude::{Children, Commands, default, Entity, EventReader, EventWriter, Query, Res, ResMut, SpriteBundle, Transform, With};
use bevy_rapier2d::dynamics::Velocity;
use bevy::log::info;
use bevy::hierarchy::BuildChildren;
use crate::asset_loader::resources::SpriteAssets;
use crate::client_networking::{ClientId, RecvHealthUpdateEvent, RecvMatchTimeEvent, RecvObjectDespawnEvent, RecvPhysObjUpdateEvent, RecvPlayerConnectEvent, RecvPlayerDataUpdateEvent, RecvPlayerLeaveEvent, RecvPlayerSpawnEvent, RecvTurretUpdateEvent, RecvYouConnectEvent};
use crate::simulation::client_sim::PlayerSpawnBuffer;
use crate::simulation::events::OnPlayerSpawnEvent;
use crate::simulation::server_sim::match_ffa::MatchTimer;
use crate::simulation::server_sim::player::{Health, Player, PlayerTurret};
use crate::utils::commands::despawn::CustomDespawnExt;
use crate::utils::prefabs::{get_player_bundle, get_turret_bundle};

#[allow(clippy::type_complexity)]
pub fn phys_obj_updater(
    mut update_event: EventReader<RecvPhysObjUpdateEvent>,
    mut query: Query<(&mut Transform,
                      Option<&mut Velocity>,
                      Option<&mut SpriteEnum>),
        With<Object>>,
    mut objects: ResMut<SyncedObjects>,
    assets: Res<SpriteAssets>,
    mut commands: Commands,
) {
    update_event.iter().for_each(|ev| match objects.objects.get(&ev.id) {
        Some(&entity) => {
            if let Ok((mut trans, vel, sprite)) = query.get_mut(entity) {
                *trans = ev.data.transform;
                if let Some(mut vel) = vel {
                    vel.linvel = ev.data.velocity;
                }
                match sprite {
                    Some(mut sprite) => *sprite = ev.data.sprite.unwrap(),
                    None => {
                        if let Some(sprite) = ev.data.sprite {
                            commands.entity(entity).insert(sprite);
                        }
                    }
                }
            }
        }
        None => {
            objects.objects.insert(ev.id, init_object(ev, &mut commands, &assets));
        }
    });
}

pub fn turr_updater(
    mut update_event: EventReader<RecvTurretUpdateEvent>,
    objects: ResMut<SyncedObjects>,
    parent_q: Query<&Children, With<Player>>,
    mut turr_q: Query<&mut Transform, With<PlayerTurret>>,
) {
    update_event.iter().for_each(|ev| {
        if let Some(&ent) = objects.objects.get(&ev.parent_id) {
            if let Ok(children) = parent_q.get(ent) {
                children.iter().for_each(|&child| {
                    if let Ok(mut turr) = turr_q.get_mut(child) {
                        turr.rotation = ev.rotation;
                    }
                });
            }
        }
    })
}

fn init_object(event: &RecvPhysObjUpdateEvent, commands: &mut Commands, assets: &SpriteAssets) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                texture: assets.get(event.data.sprite.unwrap()),
                transform: event.data.transform,
                ..default()
            },
            Velocity {
                linvel: event.data.velocity,
                ..default()
            },
            Object { id: event.id },
        ))
        .id()
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
    mut spawn_buffer: ResMut<PlayerSpawnBuffer>,
    mut spawn_writer: EventWriter<OnPlayerSpawnEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    objects: Res<SyncedObjects>,
) {
    spawn_event.iter().for_each(|e| spawn_buffer.events.push((false, e.clone())));

    spawn_buffer.events.iter_mut().for_each(|(cleanup, e)| {
        let Some(entity) = objects.objects.get(&e.object_id) else { return; };

        *cleanup = true;

        commands.entity(*entity).insert(get_player_bundle(e.player_id, Some(e.position)))
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

    spawn_buffer.events.drain_filter(|(cleanup, _)| *cleanup);
}

pub fn on_player_update(
    mut player_update_events: EventReader<RecvPlayerDataUpdateEvent>,
    mut lobby: ResMut<Lobby>,
) {
    player_update_events.iter().for_each(|e| {
        *lobby.player_data.get_mut(&e.id).unwrap() = e.data.clone();
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

pub fn on_timer_update(
    mut events: EventReader<RecvMatchTimeEvent>,
    mut match_timer: Option<ResMut<MatchTimer>>,
    mut commands: Commands,
) {
    events.iter().for_each(|e| {
        match &mut match_timer {
            Some(match_timer) => {
                match_timer.time_remaining = e.time_remaining;
            }
            None => {
                commands.insert_resource(MatchTimer {
                    time_remaining: e.time_remaining,
                    match_length: e.time_remaining,
                });
            }
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
