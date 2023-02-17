mod systems;

use bevy::prelude::{IntoSystemDescriptor, SystemLabel, SystemSet};
use bevy_renet::renet::{ClientAuthentication, RenetClient, RenetConnectionConfig};
use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::app::{App, Plugin};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy_renet::RenetClientPlugin;
use crate::networking::{NetworkPlugin, PROTOCOL_ID};
use crate::networking::client::ClientEventSysLabel::*;
use crate::networking::server::SERVER_ADDRESS;
use crate::object::ObjectSyncPlugin;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin::default())
            .add_plugin(ObjectSyncPlugin);

        app.insert_resource(new_client())
            .add_system_set(
            SystemSet::new()
                .with_run_criteria(bevy_renet::run_if_client_connected)
                .with_system(systems::client_recv.label(ClientReceive))
                .with_system(systems::client_send_input.label(ClientSend)
                    .after(ClientReceive))
        );
    }
}

#[derive(SystemLabel)]
enum ClientEventSysLabel {
    ClientReceive,
    ClientSend,
}

fn new_client() -> RenetClient {
    let server_addr = SERVER_ADDRESS.parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let auth = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    RenetClient::new(current_time, socket, connection_config, auth).unwrap()
}