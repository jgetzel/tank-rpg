use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Align2;
use crate::networking::{Lobby, PlayerData};
use crate::networking::messages::PlayerId;
use crate::scenes::AppState;

pub struct ClientLeaderboardUI;

impl Plugin for ClientLeaderboardUI {
    fn build(&self, app: &mut App) {
        app.add_system(leaderboard_ui.in_set(OnUpdate(AppState::InGame)));
    }
}

fn leaderboard_ui(
    mut contexts: EguiContexts,
    lobby: Res<Lobby>,
) {
    egui::Area::new("Leaderboard_Area")
        .anchor(Align2::LEFT_TOP, [0., 0.])
        .show(contexts.ctx_mut(), |ui| {
            ui.set_width_range(100.0..=200.0);
            egui::Frame::menu(&egui::Style::default())
                .outer_margin(10.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.set_height(5.);
                        ui.label("Leaderboard");
                    });
                    ui.separator();
                    egui::Frame::group(&egui::Style::default()).show(ui, |ui| {
                        let mut player_vec = lobby.player_data.clone().into_iter()
                            .collect::<Vec<(PlayerId, PlayerData)>>();

                        player_vec.sort_by(|(_, data_a), (_, data_b)| {
                            data_a.kills.cmp(&data_b.kills)
                        });

                        player_vec.iter().for_each(|(id, data)| {
                           ui.label(format!("Player {}: {} kills", id, data.kills));
                        });
                    });
                })
        });
}