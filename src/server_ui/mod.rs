use bevy::app::App;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_quinnet::server::{ConnectionEvent, ConnectionLostEvent, Server};
use bevy_egui::egui::Align2;
use local_ip_address::local_ip;
use crate::server_networking::SERVER_PORT;
use crate::server_ui::network_visualizer::ServerVisualizer;
use crate::utils::networking::{is_server_listening, Lobby};

mod network_visualizer;

pub struct ServerUIPlugin;

impl Plugin for ServerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(server_stats_egui.run_if(is_server_listening));
    }
}

pub fn server_stats_egui(
    mut egui_ctx: EguiContexts,
    mut client_join: EventReader<ConnectionEvent>,
    mut client_leave: EventReader<ConnectionLostEvent>,
    visualizer: Option<ResMut<ServerVisualizer<512>>>,
    mut _commands: Commands,
    lobby: Res<Lobby>,
    server: Res<Server>,
) {
    let Some(mut visualizer) = visualizer else {
        _commands.insert_resource(ServerVisualizer::<512>::default());
        return;
    };

    client_join.iter().for_each(|ConnectionEvent { id }| {
        visualizer.add_client(*id);
    });
    client_leave.iter().for_each(|ConnectionLostEvent { id }| {
        visualizer.remove_client(*id);
    });

    visualizer.update(&server);

    egui::Window::new("Server Stats")
        .anchor(Align2::LEFT_TOP, [0., 0.])
        .collapsible(true)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Server IP:");
                let server_ip = format!("{}:{}", local_ip().unwrap(), SERVER_PORT);
                ui.monospace(server_ip.clone());
                if ui.small_button("📋").clicked() {
                    ui.output_mut(|o| o.copied_text = server_ip);
                }
            });

            ui.separator();

            ui.label("Player Lobby");
            ui.group(|ui| {
                lobby.player_data.iter().for_each(|player| {
                    ui.label(format!("Player {}: Entity {:?}", player.0, player.1.clone()));
                });
            });

            ui.separator();
            visualizer.show_window(ui);
        });
}
