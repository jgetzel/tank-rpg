use bevy::app::App;
use bevy::prelude::{Commands, EventReader, EventWriter, Plugin, ResMut, SystemSet};
use bevy::log::info;
use bevy::hierarchy::BuildChildren;
use crate::networking::{Lobby, PlayerConnectEvent};
use crate::object::{Object, SyncedObjects};
use crate::player::bundles::{get_player_bundle, get_turret_bundle};
use crate::player::PlayerSpawnEvent;
use crate::prefabs::{default_background, default_camera};
use crate::scenes::AppState;

pub struct InGamePlugin;


impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(init_default)
            ).add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player_connect_listener)
        );
    }
}

fn init_default(mut commands: Commands) {
    commands.spawn(default_background());
    commands.spawn(default_camera());
}

fn player_connect_listener(
    mut join_ev: EventReader<PlayerConnectEvent>,
    mut spawn_ev_w: EventWriter<PlayerSpawnEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut objects: ResMut<SyncedObjects>,
) {
    join_ev.iter().for_each(|ev| {
        info!("Player {} Connected", ev.player_id);
        let mut ent_cmd = match objects.objects.get(&ev.object_id) {
            Some(&obj) => commands.entity(obj),
            None => {
                let ent_cmds = commands.spawn_empty();
                objects.objects.insert( ev.object_id,ent_cmds.id());
                ent_cmds
            },
        };

        let player_entity = ent_cmd.insert(get_player_bundle(ev.player_id, None))
            .insert(Object { id: ev.object_id })
            .with_children(|p| {
                p.spawn(get_turret_bundle());
            }).id();

        lobby.players.insert(ev.player_id, player_entity);
        spawn_ev_w.send(PlayerSpawnEvent { player_id: ev.player_id, object_id: ev.object_id });
    });
}
