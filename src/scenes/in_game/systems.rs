use bevy::prelude::{Commands, EventReader, EventWriter, Query, Res, ResMut};
use bevy::log::info;
use bevy::hierarchy::BuildChildren;
use bevy::time::Time;
use crate::networking::{Lobby};
use crate::networking::events::{OnPlayerConnectEvent, OnPlayerSpawnEvent};
use crate::object::{Object, SyncedObjects};
use crate::player::bundles::{get_player_bundle, get_turret_bundle};
use crate::player::{DeathEvent, Player};
use crate::prefabs::{default_background, default_camera};
use crate::scenes::in_game::{OnRespawnTimerFinish, RespawnTimer};

pub fn init_default(mut commands: Commands) {
    commands.spawn(default_background());
    commands.spawn(default_camera());
}

pub fn spawn_player_system(
    mut join_events: EventReader<OnPlayerConnectEvent>,
    mut respawn_events: EventReader<OnRespawnTimerFinish>,
    mut spawn_writer: EventWriter<OnPlayerSpawnEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut objects: ResMut<SyncedObjects>,
) {
    let events = join_events.iter().map(|e| e.player_id)
        .chain(respawn_events.iter().map(|e| e.player_id));
    events.for_each(|player_id| {
        info!("Player {} Spawned", player_id);

        let new_object = Object::new();

        let player_entity = commands.spawn(get_player_bundle(player_id, None))
            .insert(new_object)
            .with_children(|p| {
                p.spawn(get_turret_bundle());
            }).id();

        objects.objects.insert(new_object.id, player_entity);

        lobby.update_object_id(player_id, new_object.id).unwrap();

        spawn_writer.send(OnPlayerSpawnEvent {
            player_id,
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
    time: Res<Time>,
) {
    respawn_timer.map.values_mut().for_each(|v| {
        *v -= time.delta_seconds();
    });
}

pub fn dispatch_respawn_on_countdown(
    mut respawn_timer: ResMut<RespawnTimer>,
    mut respawn_event_writer: EventWriter<OnRespawnTimerFinish>,
) {
    respawn_timer.map.iter().for_each(|(player_id, time)| {
        if *time <= 0. {
            respawn_event_writer.send(OnRespawnTimerFinish { player_id: *player_id });
        }
    });

    respawn_timer.map = respawn_timer.map.clone().into_iter().filter(|(_, time)| *time > 0.).collect();
}
