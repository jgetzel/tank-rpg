use bevy::app::App;
use bevy::prelude::*;
use bevy::log::info;
use bevy::hierarchy::BuildChildren;
use bevy::utils::hashbrown::hash_map::Entry::Vacant;
use bevy::utils::HashSet;
use crate::ServerSet::ServerUpdate;
use crate::simulation::events::{OnPlayerConnectEvent, OnPlayerDisconnectEvent, OnPlayerSpawnEvent, OnRespawnTimerFinish};
use crate::simulation::server_sim::player::Player;
use crate::simulation::{Object, PlayerData, SyncedObjects};
use crate::simulation::Lobby;
use crate::simulation::server_sim::init::OnInitEvent;
use crate::utils::commands::despawn::CustomDespawnExt;
use crate::utils::networking::messages::PlayerId;
use crate::utils::prefabs::{get_player_bundle, get_turret_bundle};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                (
                    lobby_players_on_connect.before(spawn_player_system),
                    spawn_player_system.in_set(ServerUpdate),
                    despawn_on_disconnect.in_set(ServerUpdate),
                )
            );
    }
}

#[derive(Component)]
pub struct SpawnPoint;


pub fn lobby_players_on_connect(
    mut join_events: EventReader<OnPlayerConnectEvent>,
    mut lobby: ResMut<Lobby>,
) {
    join_events.iter().for_each(|e| {
        let Vacant(entry) = lobby.player_data.entry(e.player_id)
            else { return; };
        entry.insert(PlayerData::default());
    });
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_player_system(
    mut join_events: EventReader<OnPlayerConnectEvent>,
    mut respawn_events: EventReader<OnRespawnTimerFinish>,
    mut init_events: EventReader<OnInitEvent>,
    mut spawn_writer: EventWriter<OnPlayerSpawnEvent>,
    spawn_points: Query<&GlobalTransform, With<SpawnPoint>>,
    players: Query<&GlobalTransform, With<Player>>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut objects: ResMut<SyncedObjects>,
) {
    let events =
        join_events.iter().map(|e| e.player_id)
            .chain(respawn_events.iter().map(|e| e.player_id))
            .chain(init_events.iter().flat_map(|_|
                lobby.player_data.iter().map(|(&id, _)| id).collect::<Vec<PlayerId>>()
            )).collect::<HashSet<PlayerId>>();

    events.iter().for_each(|&player_id| {
        info!("Player {} Spawned", player_id);

        let spawn_position = spawn_points.iter().map(|spawn_trans| {
            let spawn_trunc = spawn_trans.translation().truncate();
            (players.iter().map(|player_trans|
                (spawn_trunc - player_trans.translation().truncate()).length())
                 .sum::<f32>(),
             spawn_trunc)
        }).max_by(|(x, _), (y, _)| x.total_cmp(y)).unwrap().1;

        let new_object = Object::new();
        let turret_object = Object::new();

        let player_entity = commands.spawn(
            get_player_bundle(player_id, Some(spawn_position)))
            .insert(new_object)
            .with_children(|p| {
                p.spawn(get_turret_bundle(p.parent_entity()))
                    .insert(turret_object);
            }).id();

        objects.objects.insert(new_object.id, player_entity);

        lobby.update_object_id(player_id, new_object.id).unwrap();

        spawn_writer.send(OnPlayerSpawnEvent {
            player_id,
            turret_object_ids: vec![turret_object.id],
            object_id: new_object.id,
            position: spawn_position,
        })
    });
}

pub fn despawn_on_disconnect(
    mut disconnect_events: EventReader<OnPlayerDisconnectEvent>,
    mut lobby: ResMut<Lobby>,
    objects: ResMut<SyncedObjects>,
    mut commands: Commands,
) {
    disconnect_events.iter().for_each(|OnPlayerDisconnectEvent { player_id } | {
        info!("Player {player_id} Disconnected");
        if let Some(PlayerData{ object_id, .. }) = lobby.player_data.remove(player_id) &&
            let Some(object_id) = object_id &&
            let Some(&entity) = objects.objects.get(&object_id)
        {
            commands.entity(entity).custom_despawn();
        }
    });
}

