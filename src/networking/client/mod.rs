mod systems;
pub mod resources;
mod client_input;

pub use crate::player::components::PlayerInput;

use bevy::prelude::{IntoSystemDescriptor, SystemLabel, SystemSet};
use bevy_renet::renet::{ClientAuthentication, RenetClient, RenetConnectionConfig};
use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::app::{App, Plugin};
use bevy_renet::RenetClientPlugin;
use serde::{Deserialize, Serialize};
use resources::RequestIdCounter;
use crate::networking::client::client_input::ClientInputPlugin;
use crate::networking::PROTOCOL_ID;
use crate::networking::client::ClientEventSysLabel::*;
use crate::networking::client::systems::{on_player_leave, prediction_move};
use crate::networking::server::SERVER_ADDRESS;
use crate::object::ObjectSyncPlugin;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin::default())
            .add_plugin(ObjectSyncPlugin)
            .add_plugin(ClientInputPlugin);

        app.insert_resource(new_client())
            .insert_resource(RequestIdCounter::default())
            .add_system_set(
            SystemSet::new()
                .with_run_criteria(bevy_renet::run_if_client_connected)
                .with_system(systems::client_recv.label(ClientReceive))
                .with_system(systems::client_send.label(ClientSend)
                    .after(ClientReceive))
                .with_system(on_player_leave)
                .with_system(prediction_move)
        );
    }
}

#[derive(SystemLabel)]
pub enum ClientEventSysLabel {
    ClientReceive,
    ClientSend,
}

pub type RequestId = u64;

#[derive(Serialize, Deserialize)]
pub struct ClientInputMessage {
    pub input: PlayerInput,
    pub request_id: RequestId,
}

fn new_client() -> RenetClient {
    let server_addr = SERVER_ADDRESS.parse().unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let auth = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let connection_config = RenetConnectionConfig::default();

    RenetClient::new(current_time, socket, connection_config, auth).unwrap()
}
