mod systems;
pub mod resources;

use bevy::prelude::{IntoSystemDescriptor, SystemLabel, SystemSet};
use bevy_renet::renet::{ClientAuthentication, RenetClient, RenetConnectionConfig};
use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::app::{App, Plugin};
use bevy_renet::RenetClientPlugin;
use serde::{Deserialize, Serialize};
use resources::RequestIdCounter;
use crate::client_input::PlayerInput;
use crate::networking::PROTOCOL_ID;
use crate::networking::client::ClientEventSysLabel::*;
use crate::networking::server::SERVER_ADDRESS;
use crate::object::ObjectSyncPlugin;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin::default())
            .add_plugin(ObjectSyncPlugin);

        app.insert_resource(new_client())
            .insert_resource(RequestIdCounter::default())
            .add_system_set(
            SystemSet::new()
                .with_run_criteria(bevy_renet::run_if_client_connected)
                .with_system(systems::client_recv.label(ClientReceive))
                .with_system(systems::client_send.label(ClientSend)
                    .after(ClientReceive))
        );
    }
}

pub type RequestId = u64;

#[derive(Serialize, Deserialize)]
pub struct ClientInputMessage {
    pub input: PlayerInput,
    pub request_id: RequestId,
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