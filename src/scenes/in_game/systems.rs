use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, ResMut};
use bevy::log::info;
use bevy::hierarchy::BuildChildren;
use bevy::time::Time;
use crate::networking::{Lobby, PlayerData};
use crate::networking::server::events::{OnPlayerConnectEvent, OnPlayerSpawnEvent};
use crate::object::{Object, SyncedObjects};
use crate::player::bundles::{get_player_bundle, get_turret_bundle};
use crate::player::{DeathEvent, Player, spawn_new_player};
use crate::prefabs::{default_background, default_camera};
use crate::scenes::in_game::{RespawnEvent, RespawnTimer};

pub fn init_default(mut commands: Commands) {
    commands.spawn(default_background());
    commands.spawn(default_camera());
}

pub fn on_connect_spawn_player(
    mut join_events: EventReader<OnPlayerConnectEvent>,
    mut spawn_writer: EventWriter<OnPlayerSpawnEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut objects: ResMut<SyncedObjects>,
) {
    join_events.iter().for_each(|ev| {
        info!("Player {} Spawned", ev.player_id);

        let new_object = Object::new();

        let player_entity = commands.spawn(get_player_bundle(ev.player_id, None))
            .insert(new_object)
            .with_children(|p| {
                p.spawn(get_turret_bundle());
            }).id();

        objects.objects.insert(new_object.id, player_entity);

        lobby.player_data.insert(ev.player_id, PlayerData::new(new_object.id));

        spawn_writer.send(OnPlayerSpawnEvent {
            player_id: ev.player_id,
            object_id: new_object.id,
        })
    });
}

pub fn start_respawn_timer_on_death(
    mut death_reader: EventReader<DeathEvent>,
    mut respawn_timer: ResMut<RespawnTimer>,
    query: Query<&Player>,
) {
    death_reader.iter().for_each(|e| {
        if let Ok(player) = query.get(e.entity) {
            info!("Starting respawn timer for {}", player.id);
            respawn_timer.map.insert(player.id, 5.0);
        }
    });
}

pub fn run_respawn_timer(
    mut respawn_timer: ResMut<RespawnTimer>,
    time: Res<Time>
) {
    respawn_timer.map.values_mut().for_each(|v| {
        *v -= time.delta_seconds();
    });
}

pub fn dispatch_respawn_on_countdown(
    mut respawn_timer: ResMut<RespawnTimer>,
    mut respawn_event_writer: EventWriter<RespawnEvent>,
) {
    respawn_timer.map.iter().for_each(|(player_id, time)| {
        if *time <= 0. {
            respawn_event_writer.send(RespawnEvent { player_id: *player_id});
        }
    });

    respawn_timer.map = respawn_timer.map.clone().into_iter().filter(|(_, time)| *time > 0.).collect();
}

pub fn respawn_reader(
    mut respawn_events: EventReader<RespawnEvent>,
    mut spawn_event_writer: EventWriter<OnPlayerSpawnEvent>,
    mut lobby: ResMut<Lobby>,
    mut commands: Commands
) {
    respawn_events.iter().for_each(|RespawnEvent { player_id} | {
        info!("Respawning Player {}", player_id);
        let new_player_entity: Entity = spawn_new_player(&mut commands, *player_id, None);
        let new_object = Object::new();
        commands.entity(new_player_entity).insert(new_object);
        lobby.player_data.insert(*player_id, PlayerData::new(new_object.id));

        spawn_event_writer.send(OnPlayerSpawnEvent {
            player_id: *player_id,
            object_id: new_object.id
        });
    })
}
