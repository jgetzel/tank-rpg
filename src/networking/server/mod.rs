mod systems;

use std::default::Default;
use bevy::app::{App, Plugin};
use bevy_renet::RenetServerPlugin;
use crate::networking::server::systems::{force_disconnect_handler, server_recv};

pub const SERVER_ADDRESS: &str = "127.0.0.1:5000";

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetServerPlugin::default())
            .insert_resource(systems::new_server())
            .add_system(server_recv)
            .add_system(systems::server_send)
            .add_system(force_disconnect_handler);

    }
}
