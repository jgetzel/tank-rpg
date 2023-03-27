mod systems;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServerPlugin;
use crate::ServerSet::*;
use crate::server_networking::systems::*;

pub const DEFAULT_SERVER_HOSTNAME: &str = "TankRPGHost"; //TODO figure out hostnames

pub const DEFAULT_SERVER_PORT: u16 = 1337;

pub struct ServerNetworkingPlugin;

impl Plugin for ServerNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(QuinnetServerPlugin::default())
            .add_system(server_recv.in_set(ServerReceive))
            .add_systems(
                (
                    server_send_reliable,
                    server_send_unreliable,
                    server_send_init_player,
                ).in_set(ServerSend));

    }
}