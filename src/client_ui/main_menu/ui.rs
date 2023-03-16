use std::net::SocketAddr;
use std::str::FromStr;
use bevy::prelude::EventWriter;
use bevy::utils::default;
use bevy_egui::egui;
use bevy_egui::egui::*;
use bevy_egui::egui::epaint::Shadow;
use once_cell::sync::Lazy;
use crate::client_ui::main_menu::{OnConnectAttempt, OnHostAttempt};
use crate::server_networking::DEFAULT_SERVER_PORT;

pub const CENTER_WIDTH: f32 = 300.;

pub static MAIN_MENU_FRAME: Lazy<Frame> = Lazy::new(||
    Frame {
        inner_margin: Margin::from(10.),
        rounding: Rounding::from(5.0),
        shadow: Shadow {
            extrusion: 4.,
            color: Color32::from_black_alpha(50),
        },
        fill: Visuals::dark().window_fill(),
        ..default()
    }
);

pub trait MainMenuExt {
    fn connect_menu(&mut self, server_ip: &mut String, connect_writer: EventWriter<OnConnectAttempt>);

    fn host_menu(&mut self, server_port: &mut String, host_writer: EventWriter<OnHostAttempt>);
}

impl MainMenuExt for Ui {
    fn connect_menu(&mut self, server_ip: &mut String, mut connect_writer: EventWriter<OnConnectAttempt>) {
        self.vertical_centered(|ui| {
            ui.menu_button(RichText::new("Join Server").heading(), |ui| {
                ui.label("Enter Server IP:");
                ui.horizontal_top(|ui| {
                    ui.text_edit_singleline(server_ip);
                    let address = SocketAddr::from_str(server_ip.as_str());
                    if ui.add_enabled(
                        SocketAddr::from_str(server_ip.as_str()).is_ok(),
                        Button::new("Connect!"),
                    ).clicked() {
                        connect_writer.send(OnConnectAttempt { address: address.unwrap() });
                    }
                });
                if ui.button("Back").clicked() {
                    ui.close_menu();
                }
            });
        });
    }

    fn host_menu(&mut self, server_port: &mut String, mut host_writer: EventWriter<OnHostAttempt>) {
        self.vertical_centered(|ui| {
            ui.menu_button(RichText::new("Host Server").heading(), |ui| {
                ui.label("Enter Port Number:");
                ui.horizontal_top(|ui| {
                    ui.add(egui::TextEdit::singleline(server_port)
                        .hint_text(DEFAULT_SERVER_PORT.to_string()));

                    let port_num = server_port.parse::<u16>();
                    if ui.add_enabled(
                        port_num.is_ok(),
                        Button::new("Host!")
                    ).clicked() {
                        host_writer.send(OnHostAttempt { port_num: port_num.unwrap()});
                    }
                });
                if ui.button("Back").clicked() {
                    ui.close_menu();
                }
            });
        });
    }
}