use bevy::prelude::{Commands, EventReader, EventWriter, NextState, Res, ResMut};
use bevy_quinnet::client::connection::{ConnectionConfiguration, ConnectionEvent};
use bevy_quinnet::client::Client;
use bevy::log::info;
use std::net::SocketAddr;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Align2, RichText};
use std::str::FromStr;
use crate::AppState;
use crate::client_ui::main_menu::{ConnectState, OnAttemptConnect, OnAttemptHost, ServerIPInput, ServerPortInput};
use crate::client_ui::main_menu::ui::{CENTER_WIDTH, MAIN_MENU_FRAME, MainMenuExt};
use crate::utils::prefabs::default_camera;

pub fn init(mut commands: Commands) {
    commands.spawn(default_camera());
}

pub fn main_menu_gui(
    mut contexts: EguiContexts,
    mut server_ip_string: ResMut<ServerIPInput>,
    mut server_port_string: ResMut<ServerPortInput>,
    connect_writer: EventWriter<OnAttemptConnect>,
    host_writer: EventWriter<OnAttemptHost>,
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
    mut egui_ctx: EguiContexts,
) {
    egui::Window::new("Connecting...")
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .collapsible(false)
        .show(egui_ctx.ctx_mut(), |_| {});
}

pub fn connect_attempt_event_listener(
    mut events: EventReader<OnAttemptConnect>,
    server_address: Res<ServerIPInput>,
    mut next_state: ResMut<NextState<ConnectState>>,
    mut client: ResMut<Client>,
) {
    if events.iter().next().is_some() {
        info!("Connection attempt tried!");
        next_state.set(ConnectState::Connecting);
        client.open_connection(
            ConnectionConfiguration::from_addrs(
                SocketAddr::from_str(server_address.0.as_str()).unwrap(),
                SocketAddr::from_str("0.0.0.0:0").unwrap()
            ),
            CertificateVerificationMode::SkipVerification
        ).unwrap();
    }
}

pub fn in_game_on_connect(
    mut connect_event: EventReader<ConnectionEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if connect_event.iter().next().is_some() {
        next_state.set(AppState::InGame);
    }
}
