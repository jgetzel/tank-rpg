use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::networking::Lobby;
use crate::object::SyncedObjects;
use crate::scenes::AppState;

pub struct ClientDebugUI;

impl Plugin for ClientDebugUI {
    fn build(&self, app: &mut App) {
        app.add_system(show_player_lobby.run_if(in_state(AppState::InGame)));
    }
}

fn show_player_lobby(
    mut egui_ctx: EguiContexts,
    lobby: Res<Lobby>,
    objects: Res<SyncedObjects>
) {
    egui::Window::new("Client Info")
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("Lobby");
            ui.group(|ui| {
                lobby.player_data.iter().for_each(|player| {
                    ui.label(format!("Player {}: Entity {:?}", player.0, player.1.clone()));
                });
            });

            ui.separator();

            ui.label("Objects");
            ui.group(|ui| {
                objects.objects.iter().for_each(|(object_id, ent)| {
                    ui.label(format!("ObjectID {}: Entity {:?}", object_id, ent));
                });
            });
        });
}