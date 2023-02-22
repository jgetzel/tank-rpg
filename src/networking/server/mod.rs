mod systems;
mod utils;

use std::default::Default;
use bevy::app::{App, Plugin};
use bevy_renet::RenetServerPlugin;
use bevy_renet::renet::{RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig};
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;
use bevy::prelude::info;
use local_ip_address::local_ip;
use crate::networking::PROTOCOL_ID;
use crate::networking::server::systems::{force_disconnect_handler, server_recv};

pub const SERVER_ADDRESS: &str = include_str!("../../../assets/config/server_addr.txt");
pub const SERVER_PORT: u16 = 2340;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetServerPlugin::default())
            .insert_resource(new_server())
            .add_system(server_recv)
            .add_system(systems::server_send_phys_obj)
            .add_system(systems::server_send_turrets)
            .add_system(force_disconnect_handler);
    }
}

fn new_server() -> RenetServer {
    let server_addr = SocketAddr::new(local_ip().unwrap(), SERVER_PORT);
    info!("Creating Server! Local IP: {:?}", server_addr);

    let server_config = ServerConfig::new(
        64,
        PROTOCOL_ID,
        server_addr,
        ServerAuthentication::Unsecure);

    let inbound_server_addr = SocketAddr::new(local_ip().unwrap(), SERVER_PORT);
    let socket = UdpSocket::bind(inbound_server_addr).unwrap();

    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}
