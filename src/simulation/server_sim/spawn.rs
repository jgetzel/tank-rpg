use bevy::app::App;
use bevy::prelude::{Commands, Component, EventReader, EventWriter, GlobalTransform, IntoSystemConfig, Plugin, Query, ResMut, With};
use bevy::log::info;
use bevy::hierarchy::BuildChildren;
use crate::ServerSet::ServerUpdate;
use crate::simulation::events::{OnPlayerConnectEvent, OnPlayerSpawnEvent, OnRespawnTimerFinish};
use crate::simulation::server_sim::player::Player;
use crate::simulation::{Object, SyncedObjects};
use crate::simulation::Lobby;
use crate::utils::prefabs::{get_player_bundle, get_turret_bundle};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_system.in_set(ServerUpdate));
    }
}

#[derive(Component)]
pub struct SpawnPoint;

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
