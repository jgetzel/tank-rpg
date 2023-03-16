mod systems;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServerPlugin;
use crate::AppState;
use crate::ServerSet::*;
use crate::server_networking::systems::*;

pub const DEFAULT_SERVER_PORT: u16 = 1337;

pub struct ServerNetworkingPlugin;

impl Plugin for ServerNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(QuinnetServerPlugin::default())
            .add_system(in_game_on_load.in_set(OnUpdate(AppState::Loading)))
            .add_system(server_start_listening.in_schedule(OnEnter(AppState::InGame)))
            .add_system(server_recv.in_set(ServerReceive))
            .add_systems(
                (
                    server_send_phys_obj,
                    server_send_turrets,
                    on_client_connect,
                    on_client_disconnect,
                    on_player_spawn,
                    update_health,
                    update_kill_death_count,
                ).in_set(ServerSend).before(on_object_despawn))
            .add_system(on_object_despawn.in_set(ServerSend));

    }
}