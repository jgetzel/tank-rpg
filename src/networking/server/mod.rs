mod systems;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_quinnet::server::{QuinnetServerPlugin, Server};
use crate::networking::server::ServerSet::{ServerReceive, ServerSend, ServerUpdate};
use crate::networking::server::systems::*;
use crate::object::ObjectId;
use crate::scenes::AppState;

pub const SERVER_PORT: u16 = 1337;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(QuinnetServerPlugin::default())
            .add_event::<ObjectDespawnEvent>()
            .configure_set(ServerReceive.before(ServerUpdate)
                .run_if(in_state(AppState::InGame)))
            .configure_set(ServerUpdate.before(ServerSend)
                .run_if(in_state(AppState::InGame)))
            .configure_set(ServerUpdate.in_base_set(CoreSet::Update))
            .configure_set(ServerSend.run_if(in_state(AppState::InGame)))
            .add_system(in_game_on_load.in_set(OnUpdate(AppState::Loading)))
            .add_system(server_start_listening.in_schedule(OnEnter(AppState::InGame)))
            .add_system(server_recv.in_set(ServerReceive))
            .add_systems(
                (
                    server_send_phys_obj,
                    server_send_turrets,
                    on_client_connect,
                    on_client_disconnect,
                ).in_set(ServerSend).before(on_object_despawn))
            .add_system(on_object_despawn.in_set(ServerSend));

        if app.is_plugin_added::<EguiPlugin>() {
            app.add_system(server_stats_egui.run_if(is_server_listening));
        }
    }
}

pub struct ObjectDespawnEvent {
    pub id: ObjectId,
}

#[allow(clippy::enum_variant_names)]
#[derive(SystemSet, Clone, Hash, Eq, PartialEq, Debug)]
pub enum ServerSet {
    ServerReceive,
    ServerUpdate,
    ServerSend,
}

fn is_server_listening(server: Res<Server>) -> bool {
    server.is_listening()
}