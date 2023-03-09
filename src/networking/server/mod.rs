mod systems;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_quinnet::server::QuinnetServerPlugin;
use crate::networking::server::systems::*;
use crate::scenes::AppState;

pub const SERVER_PORT: u16 = 1337;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(QuinnetServerPlugin::default())
            .add_system(in_game_on_load.in_set(OnUpdate(AppState::Loading)))
            .add_system(server_start_listening.in_schedule(OnEnter(AppState::InGame)))
            .add_systems(
                (
                    server_recv,
                    server_send_phys_obj,
                    server_send_turrets,
                    on_client_connect,
                    on_client_disconnect,
                ).in_set(OnUpdate(AppState::InGame))
            );

        if app.is_plugin_added::<EguiPlugin>() {
            app.add_system(server_ip_display);
        }
    }
}
