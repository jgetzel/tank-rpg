use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::AppState;
use crate::simulation::SyncedObjects;
use crate::simulation::Lobby;

pub struct ClientDebugUIPlugin;

impl Plugin for ClientDebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DebugToggle::default())
            .add_system(debug_toggle_sys)
            .add_system(show_player_lobby.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Resource, Default)]
pub struct DebugToggle {
    debug_mode: bool
}

fn debug_toggle_sys(
    inputs: Res<Input<ScanCode>>,
    mut debug_toggle: ResMut<DebugToggle>
) {
    if inputs.just_pressed(ScanCode(0x29)) {
        debug_toggle.debug_mode = !debug_toggle.debug_mode;
    };
}

fn show_player_lobby(
    mut egui_ctx: EguiContexts,
    debug_mode: Res<DebugToggle>,
    lobby: Res<Lobby>,
    objects: Res<SyncedObjects>
) {
    if !debug_mode.debug_mode { return; }

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