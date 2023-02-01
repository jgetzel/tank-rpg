use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::SystemSet;
use bevy_renet::{RenetClientPlugin, run_if_client_connected};
use tank_rpg::input_helper::{keyboard_events, mouse_position, PlayerInput};
use tank_rpg::networking::client;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RenetClientPlugin::default())
        .insert_resource(client::new_client())
        .insert_resource(PlayerInput::default())
        .add_system(keyboard_events)
        .add_system(mouse_position)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_client_connected)
                .with_system(client::client_send_input)
                .with_system(client::client_recv)
        )
        .run();
}