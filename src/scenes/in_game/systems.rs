use bevy::prelude::{Commands, EventReader, EventWriter, GlobalTransform, Query, Res, ResMut, With};
use bevy::log::info;
use bevy::hierarchy::BuildChildren;
use bevy::time::Time;
use crate::networking::Lobby;
use crate::networking::events::{OnPlayerConnectEvent, OnPlayerSpawnEvent};
use crate::object::{Object, SyncedObjects};
use crate::player::{DeathEvent, Player};
use crate::prefabs::{default_background, default_camera, get_player_bundle, get_turret_bundle, spawn_point};
use crate::scenes::in_game::{OnRespawnTimerFinish, RespawnTimer, SpawnPoint};

pub fn init_default(mut commands: Commands) {
    commands.spawn(default_background());
    commands.spawn(default_camera());
    commands.spawn(spawn_point([-500.0, 0.].into()));
    commands.spawn(spawn_point([500.0, 0.].into()));
    commands.spawn(spawn_point([0., -500.0].into()));
    commands.spawn(spawn_point([0., 500.0].into()));
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_player_system(
    mut join_events: EventReader<OnPlayerConnectEvent>,
    mut respawn_events: EventReader<OnRespawnTimerFinish>,
    mut spawn_writer: EventWriter<OnPlayerSpawnEvent>,
    spawn_points: Query<&GlobalTransform, With<SpawnPoint>>,
    players: Query<&GlobalTransform, With<Player>>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut objects: ResMut<SyncedObjects>,
) {
    let events = join_events.iter().map(|e| e.player_id)
        .chain(respawn_events.iter().map(|e| e.player_id));
    events.for_each(|player_id| {
        info!("Player {} Spawned", player_id);

        let spawn_position = spawn_points.iter().map(|spawn_trans| {
            let spawn_trunc = spawn_trans.translation().truncate();
            (players.iter().map(|player_trans|
                (spawn_trunc - player_trans.translation().truncate()).length())
                 .sum::<f32>(),
             spawn_trunc)
        }).max_by(|(x, _), (y, _)| x.total_cmp(y)).unwrap().1;

        let new_object = Object::new();

        let player_entity = commands.spawn(
            get_player_bundle(player_id, Some(spawn_position)))
            .insert(new_object)
            .with_children(|p| {
                p.spawn(get_turret_bundle());
            }).id();

        objects.objects.insert(new_object.id, player_entity);

        lobby.update_object_id(player_id, new_object.id).unwrap();

        spawn_writer.send(OnPlayerSpawnEvent {
            player_id,
            object_id: new_object.id,
            position: spawn_position,
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
