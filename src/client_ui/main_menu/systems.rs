use bevy::prelude::{Commands, EventReader, EventWriter, NextState, ResMut};
use bevy_quinnet::client::connection::{ConnectionConfiguration, ConnectionEvent};
use bevy_quinnet::client::Client;
use bevy::log::info;
use std::net::{Ipv4Addr, SocketAddr};
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Align2, RichText};
use std::str::FromStr;
use bevy_quinnet::server::{Server, ServerConfiguration};
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use crate::AppState;
use crate::client_ui::main_menu::{ConnectState, OnConnectAttempt, OnHostAttempt, ServerIPInput, ServerPortInput};
use crate::client_ui::main_menu::ui::{CENTER_WIDTH, MAIN_MENU_FRAME, MainMenuExt};
use crate::server_networking::{DEFAULT_SERVER_HOSTNAME};
use crate::utils::prefabs::default_camera;

pub fn init(mut commands: Commands) {
    commands.spawn(default_camera());
}

pub fn main_menu_gui(
    mut contexts: EguiContexts,
    mut server_ip_string: ResMut<ServerIPInput>,
    mut server_port_string: ResMut<ServerPortInput>,
    connect_writer: EventWriter<OnConnectAttempt>,
    host_writer: EventWriter<OnHostAttempt>,
) {
    egui::Area::new("Main Menu Center Area")
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .show(contexts.ctx_mut(), |ui| {
            MAIN_MENU_FRAME.show(ui, |ui| {
                ui.set_width(CENTER_WIDTH);

                ui.connect_menu(&mut server_ip_string.0, connect_writer);

                ui.vertical_centered(|ui| {
                    ui.add_space(10.);
                    ui.label(RichText::new("- OR -").strong());
                    ui.add_space(10.);
                });

                ui.host_menu(&mut server_port_string.0, host_writer);
            });
        });
}

pub fn connecting_gui(
    mut contexts: EguiContexts,
) {
    egui::Area::new("Main Menu Connecting Area")
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .show(contexts.ctx_mut(), |ui| {
            ui.label(RichText::new("Connecting...").heading());
        });
}

pub fn connect_attempt_listener(
    mut events: EventReader<OnConnectAttempt>,
    mut next_state: ResMut<NextState<ConnectState>>,
    mut client: ResMut<Client>,
) {
    events.iter().for_each(|e| {
        info!("Attempting to connect to Socket Address {}...", e.address.to_string());
        next_state.set(ConnectState::Connecting);
        client.open_connection(
            ConnectionConfiguration::from_addrs(
                e.address,
                SocketAddr::from_str("0.0.0.0:0").unwrap(),
            ),
            CertificateVerificationMode::SkipVerification,
        ).unwrap();
    });
}

pub fn host_attempt_listener(
    mut events: EventReader<OnHostAttempt>,
    mut next_state: ResMut<NextState<ConnectState>>,
    mut client: ResMut<Client>,
    mut server: ResMut<Server>,
) {
    events.iter().for_each(|e| {
        info!("Attempting to host on port {}...", e.port_num);
        next_state.set(ConnectState::StartingServer);

        server.start_endpoint(
            ServerConfiguration::from_ip(
                Ipv4Addr::new(0, 0, 0, 0).into(), e.port_num),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: DEFAULT_SERVER_HOSTNAME.to_string()
            },
        ).unwrap();

        client.open_connection(
            ConnectionConfiguration::from_addrs(
                SocketAddr::new(
                    Ipv4Addr::from_str("127.0.0.1").unwrap().into(),
                    e.port_num,
                ),
                SocketAddr::from_str("0.0.0.0:0").unwrap(),
            ),
            CertificateVerificationMode::SkipVerification,
        ).unwrap();
    })
}

pub fn in_game_on_connect(
    mut connect_event: EventReader<ConnectionEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if connect_event.iter().next().is_some() {
        next_state.set(AppState::InGame);
    }
}
