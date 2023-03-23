use crate::client_ui::main_menu::{CenterMenuState, OnConnectAttempt, OnHostAttempt};
use crate::server_networking::DEFAULT_SERVER_PORT;
use bevy::prelude::EventWriter;
use bevy::utils::default;
use bevy_egui::egui;
use bevy_egui::egui::epaint::Shadow;
use bevy_egui::egui::*;
use once_cell::sync::Lazy;
use std::net::SocketAddr;
use std::str::FromStr;

pub const CENTER_WIDTH: f32 = 300.;

pub static MAIN_MENU_FRAME: Lazy<Frame> = Lazy::new(|| Frame {
    inner_margin: Margin::from(10.),
    rounding: Rounding::from(5.0),
    shadow: Shadow {
        extrusion: 4.,
        color: Color32::from_black_alpha(50),
    },
    fill: Visuals::dark().window_fill(),
    ..default()
});

pub trait MainMenuExt {
    fn center_menu(&mut self, center_menu_state: &mut CenterMenuState);

    fn connect_menu(
        &mut self,
        center_menu_state: &mut CenterMenuState,
        server_ip: &mut String,
        connect_writer: EventWriter<OnConnectAttempt>,
    );

    fn host_menu(
        &mut self,
        center_menu_state: &mut CenterMenuState,
        server_port: &mut String,
        host_writer: EventWriter<OnHostAttempt>,
    );
}

impl MainMenuExt for Ui {
    fn center_menu(&mut self, center_menu_state: &mut CenterMenuState) {
        self.vertical_centered(|ui| {
            if ui.button(RichText::new("Join Server").heading()).clicked() {
                *center_menu_state = CenterMenuState::Join;
            }

            ui.add_space(10.);
            ui.label(RichText::new("- OR -").strong());
            ui.add_space(10.);

            if ui.button(RichText::new("Host Server").heading()).clicked() {
                *center_menu_state = CenterMenuState::Host;
            }
        });
    }

    fn connect_menu(
        &mut self,
        center_menu_state: &mut CenterMenuState,
        server_ip: &mut String,
        mut connect_writer: EventWriter<OnConnectAttempt>,
    ) {
        self.vertical_centered(|ui| {
            ui.label("Enter Server IP:");
            ui.horizontal_top(|ui| {

                let placeholder = format!("127.0.0.1:{}", DEFAULT_SERVER_PORT);
                ui.add(
                    egui::TextEdit::singleline(server_ip)
                        .hint_text(placeholder.clone())
                );

                let address = if server_ip.is_empty() {
                    SocketAddr::from_str(placeholder.as_str())
                } else { SocketAddr::from_str(server_ip.as_str()) };

                if ui
                    .add_enabled(
                        address.is_ok(),
                        Button::new("Connect!"),
                    )
                    .clicked()
                {
                    connect_writer.send(OnConnectAttempt {
                        address: address.unwrap(),
                    });
                }
            });
            if ui.button("Back").clicked() {
                *center_menu_state = CenterMenuState::Main;
            }
        });
    }

    fn host_menu(
        &mut self,
        center_menu_state: &mut CenterMenuState,
        server_port: &mut String,
        mut host_writer: EventWriter<OnHostAttempt>
    ) {
        self.vertical_centered(|ui| {
            ui.label("Enter Port Number:");
            ui.horizontal_top(|ui| {
                ui.add(
                    egui::TextEdit::singleline(server_port)
                        .hint_text(DEFAULT_SERVER_PORT.to_string()),
                );

                let port_num = if server_port.is_empty() {
                    Ok(DEFAULT_SERVER_PORT)
                }
                else { server_port.parse::<u16>() };

                if ui
                    .add_enabled(port_num.is_ok(), Button::new("Host!"))
                    .clicked()
                {
                    host_writer.send(OnHostAttempt {
                        port_num: port_num.unwrap(),
                    });
                }
            });
            if ui.button("Back").clicked() {
                *center_menu_state = CenterMenuState::Main;
            }
        });
    }
}
