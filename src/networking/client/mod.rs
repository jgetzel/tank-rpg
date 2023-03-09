mod systems;
mod client_input;
mod main_menu;

pub use crate::player::components::PlayerInput;

use bevy::prelude::*;
use bevy::app::{App, Plugin};
use bevy_quinnet::client::{Client, QuinnetClientPlugin};
use serde::{Deserialize, Serialize};
use crate::networking::client::client_input::ClientInputPlugin;
use crate::networking::client::ClientSet::*;
use crate::networking::client::main_menu::MainMenuPlugin;
use crate::networking::client::systems::*;
use crate::object::ObjectSyncPlugin;
use crate::scenes::AppState;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(QuinnetClientPlugin::default())
            .add_plugin(ObjectSyncPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(ClientInputPlugin);

        app
            .configure_set(ClientReceive.before(ClientUpdate).run_if(is_client_connected))
            .configure_set(ClientUpdate.before(ClientSend).run_if(is_client_connected))
            .configure_set(ClientSend.run_if(is_client_connected))
            .add_systems(
                (
                    client_recv.in_set(ClientReceive),
                    client_send.in_set(ClientSend)
                )
            )
            .add_systems(
                (
                    on_player_leave,
                    on_object_despawn,
                    prediction_move,
                    ).in_set(ClientUpdate)
            )
            .add_system(main_menu_on_load.in_set(OnUpdate(AppState::Loading)));
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(SystemSet, Clone, Hash, Eq, PartialEq, Debug)]
pub enum ClientSet {
    ClientReceive,
    ClientUpdate,
    ClientSend,
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    InputMessage {
        input: PlayerInput
    }
}

fn is_client_connected(client: Res<Client>) -> bool {
    if let Some(connection) = client.get_connection() {
        connection.is_connected()
    }
    else {
        false
    }
}