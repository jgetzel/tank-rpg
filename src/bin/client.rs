extern crate core;

use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{BuildChildren, Commands, DespawnRecursiveExt, Res, ResMut, SystemSet, Vec2};
use bevy_inspector_egui::egui::Shape::Vec;
use bevy_renet::renet::{ClientAuthentication, DefaultChannel, RenetClient, RenetConnectionConfig};
use bevy_renet::{RenetClientPlugin, run_if_client_connected};
use tank_rpg::assets::GameAssets;
use tank_rpg::input_helper::{keyboard_events, mouse_position, PlayerInput};
use tank_rpg::networking::{Lobby, ServerMessages};
use tank_rpg::player::{get_player_bundle, get_turret_bundle};

const PROTOCOL_ID: u64 = 7;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RenetClientPlugin::default())
        .insert_resource(new_client())
        .insert_resource(PlayerInput::default())
        .add_system_set(
            SystemSet::new()
                .label("input")
                .with_system(keyboard_events)
                .with_system(mouse_position)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_client_connected)
                .after("input")
                .with_system(client_send_input)
                .with_system(client_recv)
        )
        .run();
}

fn new_client() -> RenetClient {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let auth = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None
    };
    RenetClient::new(current_time, socket, connection_config, auth).unwrap()
}

fn client_send_input(
    input: Res<PlayerInput>,
    mut client: ResMut<RenetClient>
) {
    let input_message = bincode::serialize(&(*input)).unwrap();
    client.send_message(DefaultChannel::Reliable, input_message);
}

fn client_recv(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    assets: Res<GameAssets>
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { id } => {
                println!("Player {id} connected");
                let player_entity = commands.spawn(
                    get_player_bundle(&assets, Some(Vec2::default()))
                ).with_children(|parent| {
                    parent.spawn(get_turret_bundle(&assets));
                }).id();

                lobby.players.insert(id, player_entity);
            },
            ServerMessages::PlayerDisconnected { id } => {
                println!("Player {id} disconnected");
                if let Some(player_entity) = lobby.players.get(&id) {
                    commands.entity(*player_entity).despawn_recursive();
                }
            }
        };

    }
}